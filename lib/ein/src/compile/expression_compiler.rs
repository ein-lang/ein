use super::boolean_compiler::BooleanCompiler;
use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::type_compiler::TypeCompiler;
use super::union_tag_calculator::UnionTagCalculator;
use crate::ast::*;
use crate::types::{self, Type};
use std::convert::TryInto;
use std::sync::Arc;

pub struct ExpressionCompiler {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    union_tag_calculator: Arc<UnionTagCalculator>,
    type_compiler: Arc<TypeCompiler>,
    boolean_compiler: Arc<BooleanCompiler>,
}

impl ExpressionCompiler {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        union_tag_calculator: Arc<UnionTagCalculator>,
        type_compiler: Arc<TypeCompiler>,
        boolean_compiler: Arc<BooleanCompiler>,
    ) -> Arc<Self> {
        Self {
            reference_type_resolver,
            union_tag_calculator,
            type_compiler,
            boolean_compiler,
        }
        .into()
    }

    pub fn compile(&self, expression: &Expression) -> Result<ssf::ir::Expression, CompileError> {
        Ok(match expression {
            Expression::Application(application) => {
                let mut function = application.function();
                let mut arguments = vec![application.argument()];

                while let Expression::Application(application) = &*function {
                    function = application.function();
                    arguments.push(application.argument());
                }

                ssf::ir::FunctionApplication::new(
                    self.compile(function)?,
                    arguments
                        .iter()
                        .rev()
                        .map(|argument| self.compile(argument))
                        .collect::<Result<_, _>>()?,
                )
                .into()
            }
            Expression::Boolean(boolean) => self.boolean_compiler.compile(boolean.value()).into(),
            Expression::Case(case) => self.compile_case(case)?,
            Expression::If(if_) => ssf::ir::AlgebraicCase::new(
                self.compile(if_.condition())?,
                vec![
                    ssf::ir::AlgebraicAlternative::new(
                        ssf::ir::Constructor::new(self.type_compiler.compile_boolean(), 0),
                        vec![],
                        self.compile(if_.else_())?,
                    ),
                    ssf::ir::AlgebraicAlternative::new(
                        ssf::ir::Constructor::new(self.type_compiler.compile_boolean(), 1),
                        vec![],
                        self.compile(if_.then())?,
                    ),
                ],
                None,
            )
            .into(),
            Expression::Let(let_) => match let_.definitions()[0] {
                Definition::FunctionDefinition(_) => self.compile_let_functions(let_)?.into(),
                Definition::ValueDefinition(_) => self.compile_let_values(let_)?.into(),
            },
            Expression::None(_) => ssf::ir::ConstructorApplication::new(
                ssf::ir::Constructor::new(self.type_compiler.compile_none(), 0),
                vec![],
            )
            .into(),
            Expression::Number(number) => ssf::ir::Primitive::Float64(number.value()).into(),
            Expression::Operation(operation) => {
                let compiled = ssf::ir::Operation::new(
                    operation.operator().try_into().unwrap(),
                    self.compile(operation.lhs())?,
                    self.compile(operation.rhs())?,
                );

                match operation.operator() {
                    Operator::Add | Operator::Subtract | Operator::Multiply | Operator::Divide => {
                        compiled.into()
                    }
                    Operator::Equal
                    | Operator::NotEqual
                    | Operator::LessThan
                    | Operator::LessThanOrEqual
                    | Operator::GreaterThan
                    | Operator::GreaterThanOrEqual => {
                        self.boolean_compiler.compile_conversion(compiled)
                    }
                    Operator::And | Operator::Or => unreachable!(),
                }
            }
            Expression::RecordConstruction(record) => ssf::ir::ConstructorApplication::new(
                ssf::ir::Constructor::new(
                    self.type_compiler
                        .compile(record.type_())?
                        .into_value()
                        .unwrap()
                        .into_algebraic()
                        .unwrap(),
                    0,
                ),
                record
                    .elements()
                    .iter()
                    .map(|(_, expression)| self.compile(expression))
                    .collect::<Result<_, _>>()?,
            )
            .into(),
            Expression::RecordElementOperation(operation) => ssf::ir::AlgebraicCase::new(
                self.compile(operation.argument())?,
                vec![ssf::ir::AlgebraicAlternative::new(
                    ssf::ir::Constructor::new(
                        self.type_compiler
                            .compile(operation.type_())?
                            .into_value()
                            .unwrap()
                            .into_algebraic()
                            .unwrap(),
                        0,
                    ),
                    self.reference_type_resolver
                        .resolve(operation.type_())?
                        .to_record()
                        .unwrap()
                        .elements()
                        .keys()
                        .map(|key| format!("${}", key))
                        .collect(),
                    ssf::ir::Variable::new(format!("${}", operation.key())),
                )],
                None,
            )
            .into(),
            Expression::TypeCoercion(coercion) => {
                let from_type = self.reference_type_resolver.resolve(coercion.from())?;
                let to_type = self
                    .type_compiler
                    .compile(coercion.to())?
                    .into_value()
                    .unwrap()
                    .into_algebraic()
                    .unwrap();
                let argument = self.compile(coercion.argument())?;

                match &from_type {
                    Type::Any(_) => argument,
                    Type::Union(_) => ssf::ir::Bitcast::new(argument, to_type).into(),
                    Type::Boolean(_)
                    | Type::Function(_)
                    | Type::List(_)
                    | Type::None(_)
                    | Type::Number(_)
                    | Type::Record(_) => {
                        if coercion.to().is_any() {
                            ssf::ir::Bitcast::new(
                                ssf::ir::ConstructorApplication::new(
                                    ssf::ir::Constructor::new(
                                        self.type_compiler
                                            .compile(
                                                &types::Union::new(
                                                    vec![from_type.clone()],
                                                    coercion.to().source_information().clone(),
                                                )
                                                .into(),
                                            )?
                                            .into_value()
                                            .unwrap()
                                            .into_algebraic()
                                            .unwrap(),
                                        self.union_tag_calculator.calculate(&from_type)?,
                                    ),
                                    vec![argument],
                                ),
                                to_type,
                            )
                            .into()
                        } else {
                            ssf::ir::ConstructorApplication::new(
                                ssf::ir::Constructor::new(
                                    to_type,
                                    self.union_tag_calculator.calculate(&from_type)?,
                                ),
                                vec![argument],
                            )
                            .into()
                        }
                    }
                    Type::Reference(_) | Type::Unknown(_) | Type::Variable(_) => unreachable!(),
                }
            }
            Expression::Variable(variable) => ssf::ir::Variable::new(variable.name()).into(),
            Expression::List(_) | Expression::RecordUpdate(_) => unreachable!(),
        })
    }

    fn compile_let_functions(&self, let_: &Let) -> Result<ssf::ir::LetFunctions, CompileError> {
        let function_definitions = let_
            .definitions()
            .iter()
            .map(|definition| match definition {
                Definition::FunctionDefinition(function_definition) => Ok(function_definition),
                Definition::ValueDefinition(value_definition) => {
                    Err(CompileError::MixedDefinitionsInLet(
                        value_definition.source_information().clone(),
                    ))
                }
            })
            .collect::<Result<Vec<&FunctionDefinition>, _>>()?;

        Ok(ssf::ir::LetFunctions::new(
            function_definitions
                .iter()
                .map(|function_definition| {
                    let type_ = function_definition
                        .type_()
                        .to_function()
                        .expect("function type");

                    Ok(ssf::ir::FunctionDefinition::new(
                        function_definition.name(),
                        function_definition
                            .arguments()
                            .iter()
                            .zip(type_.arguments())
                            .map(|(name, type_)| {
                                Ok(ssf::ir::Argument::new(
                                    name.clone(),
                                    self.type_compiler.compile(type_)?,
                                ))
                            })
                            .collect::<Result<_, CompileError>>()?,
                        self.compile(function_definition.body())?,
                        self.type_compiler.compile_value(type_.last_result())?,
                    ))
                })
                .collect::<Result<Vec<_>, CompileError>>()?,
            self.compile(let_.expression())?,
        ))
    }

    fn compile_let_values(&self, let_: &Let) -> Result<ssf::ir::LetValues, CompileError> {
        let value_definitions = let_
            .definitions()
            .iter()
            .map(|definition| match definition {
                Definition::FunctionDefinition(function_definition) => {
                    Err(CompileError::MixedDefinitionsInLet(
                        function_definition.source_information().clone(),
                    ))
                }
                Definition::ValueDefinition(value_definition) => Ok(value_definition),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ssf::ir::LetValues::new(
            value_definitions
                .iter()
                .map(|value_definition| {
                    Ok(ssf::ir::ValueDefinition::new(
                        value_definition.name(),
                        self.compile(value_definition.body())?,
                        self.type_compiler.compile_value(value_definition.type_())?,
                    ))
                })
                .collect::<Result<Vec<_>, CompileError>>()?,
            self.compile(let_.expression())?,
        ))
    }

    fn compile_case(&self, case: &Case) -> Result<ssf::ir::Expression, CompileError> {
        if !case.type_().is_any() && !case.type_().is_union() {
            return Err(CompileError::CaseArgumentTypeInvalid(
                case.source_information().clone(),
            ));
        }

        let argument_type = if case.type_().is_any() {
            self.type_compiler
                .compile_union_for_case(case.alternatives().iter().map(Alternative::type_))?
        } else {
            self.type_compiler
                .compile(case.type_())?
                .into_value()
                .unwrap()
                .into_algebraic()
                .unwrap()
        };

        Ok(ssf::ir::LetValues::new(
            vec![ssf::ir::ValueDefinition::new(
                case.name(),
                self.compile(case.argument())?,
                self.type_compiler.compile_value(case.type_())?,
            )],
            ssf::ir::AlgebraicCase::new(
                if case.type_().is_any() {
                    ssf::ir::Expression::from(ssf::ir::Bitcast::new(
                        ssf::ir::Variable::new(case.name()),
                        argument_type.clone(),
                    ))
                } else {
                    ssf::ir::Variable::new(case.name()).into()
                },
                case.alternatives()
                    .iter()
                    .take_while(|alternative| !alternative.type_().is_any())
                    .map(|alternative| {
                        match self.reference_type_resolver.resolve(alternative.type_())? {
                            Type::Boolean(_)
                            | Type::Function(_)
                            | Type::List(_)
                            | Type::None(_)
                            | Type::Number(_)
                            | Type::Record(_) => Ok(vec![ssf::ir::AlgebraicAlternative::new(
                                ssf::ir::Constructor::new(
                                    argument_type.clone(),
                                    self.union_tag_calculator.calculate(alternative.type_())?,
                                ),
                                vec![case.name().into()],
                                self.compile(alternative.expression())?,
                            )]),
                            Type::Union(union_type) => {
                                let alternative_type =
                                    self.type_compiler.compile_union(&union_type)?;

                                union_type
                                    .types()
                                    .iter()
                                    .map(|type_| {
                                        Ok(ssf::ir::AlgebraicAlternative::new(
                                            ssf::ir::Constructor::new(
                                                argument_type.clone(),
                                                self.union_tag_calculator.calculate(type_)?,
                                            ),
                                            vec![case.name().into()],
                                            ssf::ir::LetValues::new(
                                                vec![ssf::ir::ValueDefinition::new(
                                                    case.name(),
                                                    ssf::ir::ConstructorApplication::new(
                                                        ssf::ir::Constructor::new(
                                                            alternative_type.clone(),
                                                            self.union_tag_calculator
                                                                .calculate(type_)?,
                                                        ),
                                                        vec![ssf::ir::Variable::new(case.name())
                                                            .into()],
                                                    ),
                                                    alternative_type.clone(),
                                                )],
                                                self.compile(alternative.expression())?,
                                            ),
                                        ))
                                    })
                                    .collect()
                            }
                            Type::Any(_)
                            | Type::Reference(_)
                            | Type::Unknown(_)
                            | Type::Variable(_) => unreachable!(),
                        }
                    })
                    .collect::<Result<Vec<_>, CompileError>>()?
                    .into_iter()
                    .flatten()
                    .collect(),
                case.alternatives()
                    .iter()
                    .find(|alternative| alternative.type_().is_any())
                    .map(|alternative| -> Result<_, CompileError> {
                        Ok(ssf::ir::DefaultAlternative::new(
                            "",
                            self.compile(alternative.expression())?,
                        ))
                    })
                    .transpose()?,
            ),
        )
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::super::boolean_compiler::BooleanCompiler;
    use super::super::error::CompileError;
    use super::super::list_literal_configuration::ListLiteralConfiguration;
    use super::super::reference_type_resolver::ReferenceTypeResolver;
    use super::super::type_compiler::TypeCompiler;
    use super::super::union_tag_calculator::UnionTagCalculator;
    use super::ExpressionCompiler;
    use crate::ast::*;
    use crate::debug::SourceInformation;
    use crate::types;
    use pretty_assertions::assert_eq;
    use std::sync::Arc;

    fn create_expression_compiler(
        module: &Module,
    ) -> (
        Arc<ExpressionCompiler>,
        Arc<TypeCompiler>,
        Arc<UnionTagCalculator>,
    ) {
        let reference_type_resolver = ReferenceTypeResolver::new(&module);
        let union_tag_calculator = UnionTagCalculator::new(reference_type_resolver.clone());
        let type_compiler = TypeCompiler::new(
            reference_type_resolver.clone(),
            union_tag_calculator.clone(),
            ListLiteralConfiguration::new(
                "emptyList",
                "concatenateLists",
                "equalLists",
                "prependToLists",
                "GenericList",
            )
            .into(),
        );
        let boolean_compiler = BooleanCompiler::new(type_compiler.clone());

        (
            ExpressionCompiler::new(
                reference_type_resolver,
                union_tag_calculator.clone(),
                type_compiler.clone(),
                boolean_compiler,
            ),
            type_compiler,
            union_tag_calculator,
        )
    }

    #[test]
    fn compile_non_variable_function_applications() {
        let (expression_compiler, type_compiler, _) = create_expression_compiler(&Module::dummy());
        let boolean_type = type_compiler.compile_boolean();

        assert_eq!(
            expression_compiler.compile(
                &Application::new(
                    If::new(
                        Boolean::new(true, SourceInformation::dummy()),
                        Variable::new("f", SourceInformation::dummy()),
                        Variable::new("g", SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    Number::new(42.0, SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into(),
            ),
            Ok(ssf::ir::FunctionApplication::new(
                ssf::ir::AlgebraicCase::new(
                    ssf::ir::ConstructorApplication::new(
                        ssf::ir::Constructor::new(boolean_type.clone(), 1),
                        vec![]
                    ),
                    vec![
                        ssf::ir::AlgebraicAlternative::new(
                            ssf::ir::Constructor::new(boolean_type.clone(), 0),
                            vec![],
                            ssf::ir::Variable::new("g")
                        ),
                        ssf::ir::AlgebraicAlternative::new(
                            ssf::ir::Constructor::new(boolean_type, 1),
                            vec![],
                            ssf::ir::Variable::new("f")
                        )
                    ],
                    None
                ),
                vec![42.0.into()]
            )
            .into())
        );
    }

    mod operation {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn compile_arithmetic_operation() {
            let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

            assert_eq!(
                expression_compiler.compile(
                    &Operation::new(
                        Operator::Add,
                        Number::new(1.0, SourceInformation::dummy()),
                        Number::new(2.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into(),
                ),
                Ok(ssf::ir::Operation::new(ssf::ir::Operator::Add, 1.0, 2.0).into())
            );
        }

        #[test]
        fn compile_number_comparison_operation() {
            let (expression_compiler, type_compiler, _) =
                create_expression_compiler(&Module::dummy());

            assert_eq!(
                expression_compiler.compile(
                    &Operation::new(
                        Operator::LessThan,
                        Number::new(1.0, SourceInformation::dummy()),
                        Number::new(2.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into(),
                ),
                Ok(ssf::ir::PrimitiveCase::new(
                    ssf::ir::Operation::new(ssf::ir::Operator::LessThan, 1.0, 2.0),
                    vec![
                        ssf::ir::PrimitiveAlternative::new(
                            ssf::ir::Primitive::Integer8(0),
                            ssf::ir::ConstructorApplication::new(
                                ssf::ir::Constructor::new(type_compiler.compile_boolean(), 0),
                                vec![],
                            ),
                        ),
                        ssf::ir::PrimitiveAlternative::new(
                            ssf::ir::Primitive::Integer8(1),
                            ssf::ir::ConstructorApplication::new(
                                ssf::ir::Constructor::new(type_compiler.compile_boolean(), 1),
                                vec![],
                            ),
                        ),
                    ],
                    None,
                )
                .into())
            );
        }
    }

    #[test]
    fn compile_let_values() {
        let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

        assert_eq!(
            expression_compiler.compile(
                &Let::new(
                    vec![ValueDefinition::new(
                        "x",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("x", SourceInformation::dummy())
                )
                .into(),
            ),
            Ok(ssf::ir::LetValues::new(
                vec![ssf::ir::ValueDefinition::new(
                    "x",
                    42.0,
                    ssf::types::Primitive::Float64,
                )],
                ssf::ir::Variable::new("x")
            )
            .into())
        );
    }

    #[test]
    fn compile_let_functions() {
        let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

        assert_eq!(
            expression_compiler.compile(
                &Let::new(
                    vec![FunctionDefinition::new(
                        "f",
                        vec!["x".into()],
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("x", SourceInformation::dummy())
                )
                .into(),
            ),
            Ok(ssf::ir::LetFunctions::new(
                vec![ssf::ir::FunctionDefinition::new(
                    "f",
                    vec![ssf::ir::Argument::new("x", ssf::types::Primitive::Float64)],
                    42.0,
                    ssf::types::Primitive::Float64,
                )],
                ssf::ir::Variable::new("x")
            )
            .into())
        );
    }

    #[test]
    fn compile_let_functions_with_recursive_functions() {
        let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

        assert_eq!(
            expression_compiler.compile(
                &Let::new(
                    vec![FunctionDefinition::new(
                        "f",
                        vec!["x".into()],
                        Application::new(
                            Variable::new("f", SourceInformation::dummy()),
                            Variable::new("x", SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("x", SourceInformation::dummy())
                )
                .into(),
            ),
            Ok(ssf::ir::LetFunctions::new(
                vec![ssf::ir::FunctionDefinition::new(
                    "f",
                    vec![ssf::ir::Argument::new("x", ssf::types::Primitive::Float64)],
                    ssf::ir::FunctionApplication::new(
                        ssf::ir::Variable::new("f"),
                        vec![ssf::ir::Variable::new("x").into()]
                    ),
                    ssf::types::Primitive::Float64,
                )],
                ssf::ir::Variable::new("x")
            )
            .into())
        );
    }

    #[test]
    fn compile_nested_let_functions() {
        let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

        assert_eq!(
            expression_compiler.compile(
                &Let::new(
                    vec![FunctionDefinition::new(
                        "f",
                        vec!["x".into()],
                        Let::new(
                            vec![FunctionDefinition::new(
                                "g",
                                vec!["y".into()],
                                Variable::new("x", SourceInformation::dummy()),
                                types::Function::new(
                                    types::Number::new(SourceInformation::dummy()),
                                    types::Number::new(SourceInformation::dummy()),
                                    SourceInformation::dummy()
                                ),
                                SourceInformation::dummy()
                            )
                            .into()],
                            Variable::new("x", SourceInformation::dummy())
                        ),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("x", SourceInformation::dummy())
                )
                .into(),
            ),
            Ok(ssf::ir::LetFunctions::new(
                vec![ssf::ir::FunctionDefinition::new(
                    "f",
                    vec![ssf::ir::Argument::new("x", ssf::types::Primitive::Float64)],
                    ssf::ir::LetFunctions::new(
                        vec![ssf::ir::FunctionDefinition::new(
                            "g",
                            vec![ssf::ir::Argument::new("y", ssf::types::Primitive::Float64)],
                            ssf::ir::Variable::new("x"),
                            ssf::types::Primitive::Float64,
                        )],
                        ssf::ir::Variable::new("x")
                    ),
                    ssf::types::Primitive::Float64,
                )],
                ssf::ir::Variable::new("x")
            )
            .into())
        );
    }

    #[test]
    fn compile_let_values_with_free_variables() {
        let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

        assert_eq!(
            expression_compiler.compile(
                &Let::new(
                    vec![ValueDefinition::new(
                        "y",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Let::new(
                        vec![FunctionDefinition::new(
                            "f",
                            vec!["x".into()],
                            Variable::new("y", SourceInformation::dummy()),
                            types::Function::new(
                                types::Number::new(SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                SourceInformation::dummy()
                            ),
                            SourceInformation::dummy()
                        )
                        .into()],
                        Variable::new("y", SourceInformation::dummy())
                    )
                )
                .into(),
            ),
            Ok(ssf::ir::LetValues::new(
                vec![ssf::ir::ValueDefinition::new(
                    "y",
                    42.0,
                    ssf::types::Primitive::Float64,
                )],
                ssf::ir::LetFunctions::new(
                    vec![ssf::ir::FunctionDefinition::new(
                        "f",
                        vec![ssf::ir::Argument::new("x", ssf::types::Primitive::Float64)],
                        ssf::ir::Variable::new("y"),
                        ssf::types::Primitive::Float64,
                    )],
                    ssf::ir::Variable::new("y")
                )
            )
            .into())
        );
    }

    #[test]
    fn compile_if_expressions() {
        let (expression_compiler, type_compiler, _) = create_expression_compiler(&Module::dummy());
        let boolean_type = type_compiler.compile_boolean();

        assert_eq!(
            expression_compiler.compile(
                &If::new(
                    Boolean::new(true, SourceInformation::dummy()),
                    Number::new(1.0, SourceInformation::dummy()),
                    Number::new(2.0, SourceInformation::dummy()),
                    SourceInformation::dummy(),
                )
                .into(),
            ),
            Ok(ssf::ir::AlgebraicCase::new(
                ssf::ir::ConstructorApplication::new(
                    ssf::ir::Constructor::new(boolean_type.clone(), 1),
                    vec![]
                ),
                vec![
                    ssf::ir::AlgebraicAlternative::new(
                        ssf::ir::Constructor::new(boolean_type.clone(), 0),
                        vec![],
                        2.0
                    ),
                    ssf::ir::AlgebraicAlternative::new(
                        ssf::ir::Constructor::new(boolean_type, 1),
                        vec![],
                        1.0
                    )
                ],
                None
            )
            .into())
        );
    }

    #[test]
    fn fail_to_compile_case_expression_with_argument_type_invalid() {
        let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

        assert_eq!(
            expression_compiler.compile(
                &Case::new(
                    "x",
                    Boolean::new(true, SourceInformation::dummy()),
                    vec![Alternative::new(
                        types::Boolean::new(SourceInformation::dummy()),
                        Variable::new("x", SourceInformation::dummy())
                    )],
                    SourceInformation::dummy(),
                )
                .into(),
            ),
            Err(CompileError::CaseArgumentTypeInvalid(
                SourceInformation::dummy().into()
            ))
        );
    }

    #[test]
    fn compile_records() {
        let type_ = types::Record::new(
            "Foo",
            vec![(
                "foo".into(),
                types::Number::new(SourceInformation::dummy()).into(),
            )]
            .into_iter()
            .collect(),
            SourceInformation::dummy(),
        );
        let (expression_compiler, _, _) =
            create_expression_compiler(&Module::from_definitions_and_type_definitions(
                vec![TypeDefinition::new("Foo", type_.clone())],
                vec![],
            ));

        assert_eq!(
            expression_compiler.compile(
                &RecordConstruction::new(
                    types::Reference::new("Foo", SourceInformation::dummy()),
                    vec![(
                        "foo".into(),
                        Number::new(42.0, SourceInformation::dummy()).into()
                    )]
                    .into_iter()
                    .collect(),
                    SourceInformation::dummy(),
                )
                .into(),
            ),
            Ok(ssf::ir::ConstructorApplication::new(
                ssf::ir::Constructor::new(
                    ssf::types::Algebraic::new(vec![ssf::types::Constructor::boxed(vec![
                        ssf::types::Primitive::Float64.into()
                    ])]),
                    0
                ),
                vec![ssf::ir::Primitive::Float64(42.0).into()]
            )
            .into())
        );
    }

    mod type_coercion {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn compile_type_coercion_of_boolean() {
            let (expression_compiler, type_compiler, union_tag_calculator) =
                create_expression_compiler(&Module::dummy());

            let union_type = types::Union::new(
                vec![
                    types::Boolean::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            );

            assert_eq!(
                expression_compiler.compile(
                    &TypeCoercion::new(
                        Boolean::new(true, SourceInformation::dummy()),
                        types::Boolean::new(SourceInformation::dummy()),
                        union_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ),
                Ok(ssf::ir::ConstructorApplication::new(
                    ssf::ir::Constructor::new(
                        type_compiler
                            .compile(&union_type.into())
                            .unwrap()
                            .into_value()
                            .unwrap()
                            .into_algebraic()
                            .unwrap(),
                        union_tag_calculator
                            .calculate(&types::Boolean::new(SourceInformation::dummy()).into())
                            .unwrap()
                    ),
                    vec![ssf::ir::ConstructorApplication::new(
                        ssf::ir::Constructor::new(type_compiler.compile_boolean(), 1),
                        vec![]
                    )
                    .into()]
                )
                .into())
            );
        }

        #[test]
        fn compile_type_coercion_of_record() {
            let (expression_compiler, type_compiler, union_tag_calculator) =
                create_expression_compiler(&Module::dummy());

            let record_type =
                types::Record::new("Foo", Default::default(), SourceInformation::dummy());
            let union_type = types::Union::new(
                vec![
                    record_type.clone().into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            );

            assert_eq!(
                expression_compiler.compile(
                    &TypeCoercion::new(
                        Variable::new("x", SourceInformation::dummy()),
                        record_type.clone(),
                        union_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ),
                Ok(ssf::ir::ConstructorApplication::new(
                    ssf::ir::Constructor::new(
                        type_compiler
                            .compile(&union_type.into())
                            .unwrap()
                            .into_value()
                            .unwrap()
                            .into_algebraic()
                            .unwrap(),
                        union_tag_calculator.calculate(&record_type.into()).unwrap()
                    ),
                    vec![ssf::ir::Variable::new("x").into()]
                )
                .into())
            );
        }

        #[test]
        fn compile_type_coercion_of_union() {
            let (expression_compiler, type_compiler, _) =
                create_expression_compiler(&Module::dummy());

            let lower_union_type = types::Union::new(
                vec![
                    types::Boolean::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            );
            let upper_union_type = types::Union::new(
                vec![
                    types::Boolean::new(SourceInformation::dummy()).into(),
                    types::Number::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into(),
                ],
                SourceInformation::dummy(),
            );

            assert_eq!(
                expression_compiler.compile(
                    &TypeCoercion::new(
                        Variable::new("x", SourceInformation::dummy()),
                        lower_union_type.clone(),
                        upper_union_type.clone(),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ),
                Ok(ssf::ir::Bitcast::new(
                    ssf::ir::Variable::new("x"),
                    type_compiler
                        .compile(&upper_union_type.into())
                        .unwrap()
                        .into_value()
                        .unwrap()
                        .into_algebraic()
                        .unwrap(),
                )
                .into())
            );
        }

        #[test]
        fn compile_type_coercion_from_any_type_to_any_type() {
            let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

            assert_eq!(
                expression_compiler.compile(
                    &TypeCoercion::new(
                        Variable::new("x", SourceInformation::dummy()),
                        types::Any::new(SourceInformation::dummy()),
                        types::Any::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ),
                Ok(ssf::ir::Variable::new("x").into())
            );
        }

        #[test]
        fn compile_type_coercion_from_non_union_type_to_any_type() {
            let (expression_compiler, type_compiler, union_tag_calculator) =
                create_expression_compiler(&Module::dummy());

            assert_eq!(
                expression_compiler.compile(
                    &TypeCoercion::new(
                        Variable::new("x", SourceInformation::dummy()),
                        types::Boolean::new(SourceInformation::dummy()),
                        types::Any::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ),
                Ok(
                    ssf::ir::Bitcast::new(
                        ssf::ir::ConstructorApplication::new(
                            ssf::ir::Constructor::new(
                                type_compiler
                                    .compile(
                                        &types::Union::new(
                                            vec![types::Boolean::new(SourceInformation::dummy())
                                                .into()],
                                            SourceInformation::dummy()
                                        )
                                        .into()
                                    )
                                    .unwrap()
                                    .into_value()
                                    .unwrap()
                                    .into_algebraic()
                                    .unwrap(),
                                union_tag_calculator
                                    .calculate(
                                        &types::Boolean::new(SourceInformation::dummy()).into()
                                    )
                                    .unwrap()
                            ),
                            vec![ssf::ir::Variable::new("x").into()]
                        ),
                        type_compiler
                            .compile(&types::Any::new(SourceInformation::dummy()).into())
                            .unwrap()
                            .into_value()
                            .unwrap()
                            .into_algebraic()
                            .unwrap(),
                    )
                    .into()
                )
            );
        }

        #[test]
        fn compile_type_coercion_from_union_type_to_any_type() {
            let (expression_compiler, type_compiler, _) =
                create_expression_compiler(&Module::dummy());

            let union_type = types::Union::new(
                vec![types::Boolean::new(SourceInformation::dummy()).into()],
                SourceInformation::dummy(),
            );

            assert_eq!(
                expression_compiler.compile(
                    &TypeCoercion::new(
                        Variable::new("x", SourceInformation::dummy()),
                        union_type,
                        types::Any::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )
                    .into(),
                ),
                Ok(ssf::ir::Bitcast::new(
                    ssf::ir::Variable::new("x"),
                    type_compiler
                        .compile(&types::Any::new(SourceInformation::dummy()).into())
                        .unwrap()
                        .into_value()
                        .unwrap()
                        .into_algebraic()
                        .unwrap(),
                )
                .into())
            );
        }
    }
}
