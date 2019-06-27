use super::error::CompileError;
use super::function_compiler::FunctionCompiler;
use super::llvm;
use super::type_compiler::TypeCompiler;
use crate::ast;
use std::collections::HashMap;

pub struct ExpressionCompiler<'a> {
    builder: &'a llvm::Builder,
    function_compiler: &'a FunctionCompiler<'a>,
    type_compiler: &'a TypeCompiler,
}

impl<'a> ExpressionCompiler<'a> {
    pub fn new(
        builder: &'a llvm::Builder,
        function_compiler: &'a FunctionCompiler,
        type_compiler: &'a TypeCompiler,
    ) -> Self {
        Self {
            builder,
            function_compiler,
            type_compiler,
        }
    }

    pub fn compile(
        &self,
        expression: &ast::Expression,
        variables: &HashMap<String, llvm::Value>,
    ) -> Result<llvm::Value, CompileError> {
        match expression {
            ast::Expression::Application(application) => {
                let closure = self.compile(application.function(), variables)?;

                let mut arguments = vec![self.builder.build_bit_cast(
                    self.builder.build_gep(
                        closure,
                        &[
                            llvm::const_int(llvm::Type::i32(), 0),
                            llvm::const_int(llvm::Type::i32(), 1),
                        ],
                    ),
                    llvm::Type::pointer(self.type_compiler.compile_unsized_environment()),
                )];

                for argument in application.arguments() {
                    arguments.push(self.compile(argument, variables)?);
                }

                Ok(self.builder.build_call(
                    self.builder.build_load(self.builder.build_gep(
                        closure,
                        &[
                            llvm::const_int(llvm::Type::i32(), 0),
                            llvm::const_int(llvm::Type::i32(), 0),
                        ],
                    )),
                    &arguments,
                ))
            }
            ast::Expression::LetFunctions(let_functions) => {
                let mut variables = variables.clone();

                for definition in let_functions.definitions() {
                    let closure_type = self.type_compiler.compile_closure(definition);

                    variables.insert(
                        definition.name().into(),
                        self.builder.build_bit_cast(
                            self.builder.build_malloc(closure_type.size()),
                            llvm::Type::pointer(closure_type),
                        ),
                    );
                }

                for definition in let_functions.definitions() {
                    self.builder.build_store(
                        self.function_compiler.compile(definition)?,
                        self.builder.build_gep(
                            variables[definition.name()],
                            &[
                                llvm::const_int(llvm::Type::i32(), 0),
                                llvm::const_int(llvm::Type::i32(), 0),
                            ],
                        ),
                    );

                    for (index, value) in definition
                        .environment()
                        .iter()
                        .map(|argument| variables.get(argument.name()).map(|value| value.clone()))
                        .collect::<Option<Vec<_>>>()
                        .ok_or(CompileError::new("variable not found"))?
                        .iter()
                        .enumerate()
                    {
                        let pointer = self.builder.build_gep(
                            variables[definition.name()],
                            &[
                                llvm::const_int(llvm::Type::i32(), 0),
                                llvm::const_int(llvm::Type::i32(), 1),
                                llvm::const_int(llvm::Type::i32(), index as u64),
                            ],
                        );

                        self.builder.build_store(
                            self.builder
                                .build_bit_cast(*value, pointer.type_().element()),
                            pointer,
                        );
                    }
                }

                self.compile(let_functions.expression(), &variables)
            }
            ast::Expression::LetValues(let_values) => {
                let mut variables = variables.clone();

                for definition in let_values.definitions() {
                    variables.insert(
                        definition.name().into(),
                        self.compile(definition.body(), &variables)?,
                    );
                }

                self.compile(let_values.expression(), &variables)
            }
            ast::Expression::Number(number) => Ok(llvm::const_real(llvm::Type::double(), *number)),
            ast::Expression::Operation(operation) => {
                let lhs = self.compile(operation.lhs(), variables)?;
                let rhs = self.compile(operation.rhs(), variables)?;

                Ok(match operation.operator() {
                    ast::Operator::Add => self.builder.build_fadd(lhs, rhs),
                    ast::Operator::Subtract => self.builder.build_fsub(lhs, rhs),
                    ast::Operator::Multiply => self.builder.build_fmul(lhs, rhs),
                    ast::Operator::Divide => self.builder.build_fdiv(lhs, rhs),
                })
            }
            ast::Expression::Variable(name) => match variables.get(name) {
                Some(value) => Ok(self.unwrap_value(*value)),
                None => Err(CompileError::new("variable not found")),
            },
        }
    }

    fn unwrap_value(&self, value: llvm::Value) -> llvm::Value {
        if value.type_().kind() == llvm::TypeKind::Pointer {
            match value.type_().element().kind() {
                llvm::TypeKind::Double => self.builder.build_load(value),
                _ => value,
            }
        } else {
            value
        }
    }
}
