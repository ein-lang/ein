use super::super::ast;
use super::error::CompileError;
use super::expression_compiler::ExpressionCompiler;
use super::function_compiler::FunctionCompiler;
use super::initializer_configuration::InitializerConfiguration;
use super::initializer_sorter::InitializerSorter;
use super::llvm;
use super::type_compiler::TypeCompiler;
use crate::types;
use crate::types::Type;
use std::collections::HashMap;

pub struct ModuleCompiler<'a> {
    module: &'a mut llvm::Module,
    ast_module: &'a ast::Module,
    type_compiler: &'a TypeCompiler,
    global_variables: HashMap<String, llvm::Value>,
    initializers: HashMap<String, llvm::Value>,
    initializer_configuration: &'a InitializerConfiguration,
}

impl<'a> ModuleCompiler<'a> {
    pub fn new(
        module: &'a mut llvm::Module,
        ast_module: &'a ast::Module,
        type_compiler: &'a TypeCompiler,
        initializer_configuration: &'a InitializerConfiguration,
    ) -> ModuleCompiler<'a> {
        ModuleCompiler {
            module,
            ast_module,
            type_compiler,
            global_variables: HashMap::new(),
            initializers: HashMap::new(),
            initializer_configuration,
        }
    }

    pub fn compile(&mut self) -> Result<(), CompileError> {
        self.module.declare_intrinsics();

        for declaration in self.ast_module.declarations() {
            match declaration.type_() {
                Type::Function(function_type) => {
                    self.declare_function(declaration.name(), function_type)
                }
                Type::Index(_) => Err(CompileError::InvalidTypeIndex)?,
                Type::Value(value_type) => {
                    self.declare_global_variable(declaration.name(), value_type)
                }
            }
        }

        for definition in self.ast_module.definitions() {
            match definition {
                ast::Definition::FunctionDefinition(function_definition) => {
                    self.declare_function(function_definition.name(), function_definition.type_())
                }
                ast::Definition::ValueDefinition(value_definition) => {
                    self.declare_global_variable(value_definition.name(), value_definition.type_())
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

        self.compile_module_initializer()?;

        llvm::verify_module(self.module);

        Ok(())
    }

    fn declare_function(&mut self, name: &str, type_: &types::Function) {
        self.global_variables.insert(
            name.into(),
            self.module
                .add_global(name, self.type_compiler.compile_unsized_closure(type_)),
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

    fn declare_global_variable(&mut self, name: &str, value_type: &types::Value) {
        self.global_variables.insert(
            name.into(),
            self.module
                .add_global(name, self.type_compiler.compile_value(value_type)),
        );
    }

    fn compile_global_variable(
        &mut self,
        value_definition: &ast::ValueDefinition,
    ) -> Result<(), CompileError> {
        let global_variable = self.global_variables[value_definition.name()];
        global_variable.set_initializer(llvm::get_undef(global_variable.type_().element()));

        let initializer = self.module.add_function(
            &Self::get_initializer_name(value_definition.name()),
            llvm::Type::function(llvm::Type::void(), &[]),
        );

        let builder = llvm::Builder::new(initializer);
        builder.position_at_end(builder.append_basic_block("entry"));
        builder.build_store(
            ExpressionCompiler::new(
                &builder,
                &mut FunctionCompiler::new(self.module, self.type_compiler, &self.global_variables),
                &self.type_compiler,
            )
            .compile(&value_definition.body(), &self.global_variables)?,
            global_variable,
        );
        builder.build_ret_void();

        llvm::verify_function(initializer);
        self.initializers
            .insert(value_definition.name().into(), initializer);

        Ok(())
    }

    fn compile_module_initializer(&mut self) -> Result<(), CompileError> {
        let flag = self.module.add_global(
            &[self.initializer_configuration.name(), "$initialized"].concat(),
            llvm::Type::i1(),
        );
        flag.set_initializer(llvm::const_int(llvm::Type::i1(), 0));

        let initializer = self.module.add_function(
            self.initializer_configuration.name(),
            llvm::Type::function(llvm::Type::void(), &[]),
        );

        let builder = llvm::Builder::new(initializer);

        builder.position_at_end(builder.append_basic_block("entry"));
        let initialize_block = builder.append_basic_block("initialize");
        let end_block = builder.append_basic_block("end");

        builder.build_cond_br(builder.build_load(flag), end_block, initialize_block);
        builder.position_at_end(initialize_block);

        for dependent_initializer_name in
            self.initializer_configuration.dependent_initializer_names()
        {
            self.module
                .declare_function(dependent_initializer_name, llvm::Type::void(), &[]);
            builder.build_call_with_name(dependent_initializer_name, &[]);
        }

        for name in InitializerSorter::sort(&self.ast_module)? {
            builder.build_call(self.initializers[name], &[]);
        }

        builder.build_store(llvm::const_int(llvm::Type::i1(), 1), flag);

        builder.build_br(end_block);
        builder.position_at_end(end_block);

        builder.build_ret_void();

        Ok(())
    }

    fn get_initializer_name(name: &str) -> String {
        [name, ".$init"].concat()
    }
}
