use super::error::CompileError;
use super::llvm;
use crate::ast;
use std::collections::HashMap;

pub struct ExpressionCompiler<'a> {
    builder: &'a llvm::Builder,
    variables: &'a HashMap<String, llvm::Value>,
}

impl<'a> ExpressionCompiler<'a> {
    pub fn new(builder: &'a llvm::Builder, variables: &'a HashMap<String, llvm::Value>) -> Self {
        Self { builder, variables }
    }

    pub fn compile(&self, expression: &ast::Expression) -> Result<llvm::Value, CompileError> {
        unsafe {
            match expression {
                ast::Expression::Application(application) => {
                    let mut function = application.function();
                    let mut ast_arguments = vec![application.argument()];

                    while let ast::Expression::Application(application) = &*function {
                        function = application.function();
                        ast_arguments.push(application.argument());
                    }

                    ast_arguments.reverse();

                    let closure = self.compile(function)?;

                    let mut arguments = vec![self.builder.build_gep(
                        self.builder.build_bit_cast(
                            closure,
                            llvm::Type::pointer(llvm::Type::struct_(&[
                                closure.type_().element().struct_elements()[0],
                                llvm::Type::i8(),
                            ])),
                        ),
                        &[
                            llvm::const_int(llvm::Type::i32(), 0),
                            llvm::const_int(llvm::Type::i32(), 1),
                        ],
                    )];

                    for ast_argument in ast_arguments {
                        arguments.push(self.compile(ast_argument)?);
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
                ast::Expression::Number(number) => {
                    Ok(llvm::const_real(llvm::Type::double(), *number))
                }
                ast::Expression::Operation(operation) => {
                    let lhs = self.compile(operation.lhs())?;
                    let rhs = self.compile(operation.rhs())?;

                    Ok(match operation.operator() {
                        ast::Operator::Add => self.builder.build_fadd(lhs, rhs),
                        ast::Operator::Subtract => self.builder.build_fsub(lhs, rhs),
                        ast::Operator::Multiply => self.builder.build_fmul(lhs, rhs),
                        ast::Operator::Divide => self.builder.build_fdiv(lhs, rhs),
                    })
                }
                ast::Expression::Variable(name) => match self.variables.get(name) {
                    Some(value) => Ok(self.unwrap_value(*value)),
                    None => Err(CompileError::new("variable not found")),
                },
            }
        }
    }

    unsafe fn unwrap_value(&self, value: llvm::Value) -> llvm::Value {
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
