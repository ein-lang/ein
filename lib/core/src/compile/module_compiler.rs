use super::super::ast;
use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::function_compiler::FunctionCompiler;
use super::llvm;
use super::type_compiler::TypeCompiler;
use std::collections::HashMap;

const GLOBAL_INITIALIZER_NAME: &str = "sloth_init";

pub struct ModuleCompiler<'a> {
    module: &'a mut llvm::Module,
    ast_module: &'a ast::Module,
    type_compiler: &'a TypeCompiler,
    global_variables: HashMap<String, llvm::Value>,
    initializers: Vec<llvm::Value>,
}

impl<'a> ModuleCompiler<'a> {
    pub fn new(
        module: &'a mut llvm::Module,
        ast_module: &'a ast::Module,
        type_compiler: &'a TypeCompiler,
    ) -> ModuleCompiler<'a> {
        ModuleCompiler {
            module,
            ast_module,
            type_compiler,
            global_variables: HashMap::new(),
            initializers: vec![],
        }
    }

    pub fn compile(&mut self) -> Result<(), CompileError> {
        self.module.declare_intrinsics();

        for definition in self.ast_module.definitions() {
            match definition {
                ast::Definition::FunctionDefinition(function_definition) => {
                    self.declare_function(function_definition)
                }
                ast::Definition::ValueDefinition(value_definition) => {
                    self.declare_global_variable(value_definition)
                }
            }
        }

        for definition in self.ast_module.definitions() {
            match definition {
                ast::Definition::FunctionDefinition(function_definition) => {
                    self.compile_function(function_definition)?
                }
                ast::Definition::ValueDefinition(value_definition) => {
                    self.compile_global_variable(value_definition)?
                }
            }
        }

        self.compile_global_initializer();

        llvm::verify_module(self.module);

        Ok(())
    }

    fn declare_function(&mut self, function_definition: &ast::FunctionDefinition) {
        self.global_variables.insert(
            function_definition.name().into(),
            self.module.add_global(
                function_definition.name(),
                self.type_compiler.compile_closure(function_definition),
            ),
        );
    }

    fn compile_function(
        &mut self,
        function_definition: &ast::FunctionDefinition,
    ) -> Result<(), CompileError> {
        self.global_variables[function_definition.name()].set_initializer(llvm::const_struct(&[
            FunctionCompiler::new(self.module, self.type_compiler, &self.global_variables)
                .compile(function_definition)?,
            llvm::const_struct(&[]),
        ]));

        Ok(())
    }

    fn declare_global_variable(&mut self, value_definition: &ast::ValueDefinition) {
        self.global_variables.insert(
            value_definition.name().into(),
            self.module.add_global(
                value_definition.name(),
                self.type_compiler.compile_value(value_definition.type_()),
            ),
        );
    }

    fn compile_global_variable(
        &mut self,
        value_definition: &ast::ValueDefinition,
    ) -> Result<(), CompileError> {
        let global_variable = self.global_variables[value_definition.name()];
        global_variable.set_initializer(llvm::get_undef(global_variable.type_().element()));

        let initializer = self.module.add_function(
            &Self::generate_initializer_name(value_definition.name()),
            llvm::Type::function(llvm::Type::void(), &[]),
        );

        let builder = llvm::Builder::new(initializer);
        builder.position_at_end(builder.append_basic_block("entry"));
        builder.build_store(
            ExpressionCompiler::new(
                &builder,
                &FunctionCompiler::new(self.module, self.type_compiler, &self.global_variables),
                &self.type_compiler,
            )
            .compile(&value_definition.body(), &self.global_variables)?,
            global_variable,
        );
        builder.build_ret_void();

        llvm::verify_function(initializer);
        self.initializers.push(initializer);

        Ok(())
    }

    fn compile_global_initializer(&mut self) {
        let initializer = self.module.add_function(
            GLOBAL_INITIALIZER_NAME,
            llvm::Type::function(llvm::Type::void(), &[]),
        );
        let builder = llvm::Builder::new(initializer);
        builder.position_at_end(builder.append_basic_block("entry"));

        // TODO: Sort initializers topologically.
        for initializer in &self.initializers {
            builder.build_call(*initializer, &[]);
        }

        builder.build_ret_void();
    }

    fn generate_initializer_name(name: &str) -> String {
        [name, ".$init"].concat()
    }
}
