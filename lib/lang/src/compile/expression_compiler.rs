use super::boolean_compiler::BooleanCompiler;
use super::error::CompileError;
use super::last_result_type_calculator::LastResultTypeCalculator;
use super::none_compiler::NoneCompiler;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::string_type_configuration::StringTypeConfiguration;
use super::transform::{
    BooleanOperationTransformer, EqualOperationTransformer, FunctionTypeCoercionTransformer,
    LetErrorTransformer, ListCaseTransformer, ListLiteralTransformer, ListTypeCoercionTransformer,
    NotEqualOperationTransformer,
};
use super::type_compiler::TypeCompiler;
use super::union_tag_calculator::UnionTagCalculator;
use super::variable_compiler::VariableCompiler;
use crate::ast::*;
use crate::types::{self, Type};
use std::sync::Arc;

pub struct ExpressionCompilerSet {
    pub boolean_compiler: Arc<BooleanCompiler>,
    pub none_compiler: Arc<NoneCompiler>,
    pub variable_compiler: Arc<VariableCompiler>,
}

pub struct ExpressionTransformerSet {
    pub equal_operation_transformer: Arc<EqualOperationTransformer>,
    pub not_equal_operation_transformer: Arc<NotEqualOperationTransformer>,
    pub list_literal_transformer: Arc<ListLiteralTransformer>,
    pub boolean_operation_transformer: Arc<BooleanOperationTransformer>,
    pub function_type_coercion_transformer: Arc<FunctionTypeCoercionTransformer>,
    pub list_type_coercion_transformer: Arc<ListTypeCoercionTransformer>,
    pub list_case_transformer: Arc<ListCaseTransformer>,
    pub let_error_transformer: Arc<LetErrorTransformer>,
}

pub struct ExpressionCompiler {
    expression_compiler_set: Arc<ExpressionCompilerSet>,
    expression_transformer_set: Arc<ExpressionTransformerSet>,
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    last_result_type_calculator: Arc<LastResultTypeCalculator>,
    union_tag_calculator: Arc<UnionTagCalculator>,
    type_compiler: Arc<TypeCompiler>,
    string_type_configuration: Arc<StringTypeConfiguration>,
}

impl ExpressionCompiler {
    pub fn new(
        expression_compiler_set: Arc<ExpressionCompilerSet>,
        expression_transformer_set: Arc<ExpressionTransformerSet>,
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        last_result_type_calculator: Arc<LastResultTypeCalculator>,
        union_tag_calculator: Arc<UnionTagCalculator>,
        type_compiler: Arc<TypeCompiler>,
        string_type_configuration: Arc<StringTypeConfiguration>,
    ) -> Arc<Self> {
        Self {
            expression_compiler_set,
            expression_transformer_set,
            reference_type_resolver,
            last_result_type_calculator,
            union_tag_calculator,
            type_compiler,
            string_type_configuration,
        }
        .into()
    }

    pub fn compile(&self, expression: &Expression) -> Result<ssf::ir::Expression, CompileError> {
        Ok(match expression {
            Expression::Application(application) => ssf::ir::FunctionApplication::new(
                self.compile(application.function())?,
                self.compile(application.argument())?,
            )
            .into(),
            Expression::Boolean(boolean) => self
                .expression_compiler_set
                .boolean_compiler
                .compile(boolean.value())
                .into(),
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
            Expression::Let(let_) => self.compile_let(let_)?,
            Expression::LetError(let_) => self.compile_let_error(let_)?,
            Expression::LetRecursive(let_) => self.compile_let_recursive(let_)?.into(),
            Expression::None(_) => self.expression_compiler_set.none_compiler.compile().into(),
            Expression::List(list) => self.compile(
                &self
                    .expression_transformer_set
                    .list_literal_transformer
                    .transform(list)?,
            )?,
            Expression::ListCase(case) => self.compile(
                &self
                    .expression_transformer_set
                    .list_case_transformer
                    .transform(case)?,
            )?,
            Expression::Number(number) => ssf::ir::Primitive::Float64(number.value()).into(),
            Expression::Operation(operation) => match operation {
                Operation::Arithmetic(operation) => ssf::ir::PrimitiveOperation::new(
                    operation.operator().into(),
                    self.compile(operation.lhs())?,
                    self.compile(operation.rhs())?,
                )
                .into(),
                Operation::Boolean(operation) => self.compile(
                    &self
                        .expression_transformer_set
                        .boolean_operation_transformer
                        .transform(operation),
                )?,
                Operation::Equality(operation) => match operation.operator() {
                    EqualityOperator::Equal => {
                        match self.reference_type_resolver.resolve(operation.type_())? {
                            Type::Number(_) => self
                                .expression_compiler_set
                                .boolean_compiler
                                .compile_conversion(ssf::ir::PrimitiveOperation::new(
                                    operation.operator().into(),
                                    self.compile(operation.lhs())?,
                                    self.compile(operation.rhs())?,
                                )),
                            Type::String(_) => ssf::ir::FunctionApplication::new(
                                ssf::ir::FunctionApplication::new(
                                    ssf::ir::Variable::new(
                                        &self.string_type_configuration.equal_function_name,
                                    ),
                                    self.compile(operation.lhs())?,
                                ),
                                self.compile(operation.rhs())?,
                            )
                            .into(),
                            _ => self.compile(
                                &self
                                    .expression_transformer_set
                                    .equal_operation_transformer
                                    .transform(operation)?,
                            )?,
                        }
                    }
                    EqualityOperator::NotEqual => self.compile(
                        &self
                            .expression_transformer_set
                            .not_equal_operation_transformer
                            .transform(operation),
                    )?,
                },
                Operation::Order(operation) => self
                    .expression_compiler_set
                    .boolean_compiler
                    .compile_conversion(ssf::ir::PrimitiveOperation::new(
                        operation.operator().into(),
                        self.compile(operation.lhs())?,
                        self.compile(operation.rhs())?,
                    )),
                Operation::Pipe(operation) => self.compile(
                    &Application::new(
                        operation.rhs().clone(),
                        operation.lhs().clone(),
                        operation.source_information().clone(),
                    )
                    .into(),
                )?,
            },
            Expression::RecordConstruction(record) => ssf::ir::ConstructorApplication::new(
                ssf::ir::Constructor::new(
                    self.type_compiler
                        .compile(record.type_())?
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
            Expression::RecordElementOperation(operation) => {
                let algebraic_type = self
                    .type_compiler
                    .compile(operation.type_())?
                    .into_algebraic()
                    .unwrap();

                ssf::ir::AlgebraicCase::new(
                    self.compile(operation.argument())?,
                    vec![ssf::ir::AlgebraicAlternative::new(
                        ssf::ir::Constructor::new(algebraic_type, 0),
                        self.reference_type_resolver
                            .resolve_to_record(operation.type_())?
                            .unwrap()
                            .elements()
                            .keys()
                            .map(|key| {
                                if key == operation.key() {
                                    operation.variable().into()
                                } else {
                                    format!("${}", key)
                                }
                            })
                            .collect(),
                        self.compile(operation.expression())?,
                    )],
                    None,
                )
                .into()
            }
            Expression::String(string) => {
                let length = string.value().as_bytes().len();

                ssf::ir::ConstructorApplication::new(
                    ssf::ir::Constructor::new(self.type_compiler.compile_string(), 0),
                    vec![
                        ssf::ir::Bitcast::new(
                            ssf::ir::ConstructorApplication::new(
                                ssf::ir::Constructor::new(
                                    self.type_compiler.compile_string_buffer(length),
                                    0,
                                ),
                                string
                                    .value()
                                    .as_bytes()
                                    .iter()
                                    .map(|byte| ssf::ir::Primitive::Integer8(*byte).into())
                                    .collect(),
                            ),
                            ssf::types::Primitive::Integer64,
                        )
                        .into(),
                        ssf::ir::Primitive::Integer64(length as u64).into(),
                    ],
                )
                .into()
            }
            Expression::TypeCoercion(coercion) => {
                if self.reference_type_resolver.is_list(coercion.to())? {
                    self.compile(
                        &self
                            .expression_transformer_set
                            .list_type_coercion_transformer
                            .transform(coercion)?,
                    )?
                } else if self.reference_type_resolver.is_function(coercion.to())? {
                    self.compile(
                        &self
                            .expression_transformer_set
                            .function_type_coercion_transformer
                            .transform(coercion)?,
                    )?
                } else {
                    // Coerce to union or Any types.

                    let from_type = self.reference_type_resolver.resolve(coercion.from())?;
                    let to_type = self
                        .type_compiler
                        .compile(coercion.to())?
                        .into_algebraic()
                        .unwrap();
                    let argument = self.compile(coercion.argument())?;

                    match &from_type {
                        Type::Any(_) => argument,
                        Type::Boolean(_)
                        | Type::Function(_)
                        | Type::List(_)
                        | Type::None(_)
                        | Type::Number(_)
                        | Type::Record(_)
                        | Type::String(_) => {
                            if self.reference_type_resolver.is_any(coercion.to())? {
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
                        Type::Union(_) => ssf::ir::Bitcast::new(argument, to_type).into(),
                        Type::Reference(_) | Type::Unknown(_) | Type::Variable(_) => unreachable!(),
                    }
                }
            }
            Expression::Variable(variable) => self
                .expression_compiler_set
                .variable_compiler
                .compile(&variable),
            Expression::RecordUpdate(_) => unreachable!(),
        })
    }

    fn compile_let_recursive(
        &self,
        let_: &LetRecursive,
    ) -> Result<ssf::ir::LetRecursive, CompileError> {
        Ok(ssf::ir::LetRecursive::new(
            let_.definitions()
                .iter()
                .map(|function_definition| {
                    let type_ = self
                        .reference_type_resolver
                        .resolve_to_function(function_definition.type_())?
                        .unwrap();

                    Ok(ssf::ir::Definition::new(
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
                        self.type_compiler.compile(
                            &self.last_result_type_calculator.calculate(
                                function_definition.type_(),
                                function_definition.arguments().len(),
                            )?,
                        )?,
                    ))
                })
                .collect::<Result<Vec<_>, CompileError>>()?,
            self.compile(let_.expression())?,
        ))
    }

    fn compile_let(&self, let_: &Let) -> Result<ssf::ir::Expression, CompileError> {
        let_.definitions().iter().rev().fold(
            self.compile(let_.expression()),
            |expression, variable_definition| {
                Ok(ssf::ir::Let::new(
                    variable_definition.name(),
                    self.type_compiler.compile(variable_definition.type_())?,
                    self.compile(variable_definition.body())?,
                    expression?,
                )
                .into())
            },
        )
    }

    fn compile_let_error(&self, let_: &LetError) -> Result<ssf::ir::Expression, CompileError> {
        self.compile(
            &self
                .expression_transformer_set
                .let_error_transformer
                .transform(let_)?,
        )
    }

    fn compile_case(&self, case: &Case) -> Result<ssf::ir::Expression, CompileError> {
        if !self.reference_type_resolver.is_any(case.type_())?
            && !self.reference_type_resolver.is_union(case.type_())?
        {
            return Err(CompileError::CaseArgumentTypeInvalid(
                case.source_information().clone(),
            ));
        }

        let argument_type = if self.reference_type_resolver.is_any(case.type_())? {
            self.type_compiler
                .compile_union_for_case(case.alternatives().iter().map(Alternative::type_))?
        } else {
            self.type_compiler
                .compile(case.type_())?
                .into_algebraic()
                .unwrap()
        };

        Ok(ssf::ir::Let::new(
            case.name(),
            self.type_compiler.compile(case.type_())?,
            self.compile(case.argument())?,
            ssf::ir::AlgebraicCase::new(
                if self.reference_type_resolver.is_any(case.type_())? {
                    ssf::ir::Expression::from(ssf::ir::Bitcast::new(
                        ssf::ir::Variable::new(case.name()),
                        argument_type.clone(),
                    ))
                } else {
                    ssf::ir::Variable::new(case.name()).into()
                },
                case.alternatives()
                    .iter()
                    .map(|alternative| {
                        self.compile_alternative(alternative, &argument_type, case.name())
                    })
                    .collect::<Result<Vec<Option<Vec<_>>>, CompileError>>()?
                    .into_iter()
                    .fuse()
                    .flatten()
                    .flatten()
                    .collect(),
                case.alternatives()
                    .iter()
                    .map(|alternative| -> Result<_, CompileError> {
                        Ok(
                            if self.reference_type_resolver.is_any(alternative.type_())? {
                                Some(self.compile(alternative.expression())?)
                            } else {
                                None
                            },
                        )
                    })
                    .collect::<Result<Vec<Option<_>>, _>>()?
                    .into_iter()
                    .flatten()
                    .next(),
            ),
        )
        .into())
    }

    fn compile_alternative(
        &self,
        alternative: &Alternative,
        argument_type: &ssf::types::Algebraic,
        variable_name: &str,
    ) -> Result<Option<Vec<ssf::ir::AlgebraicAlternative>>, CompileError> {
        Ok(
            match self.reference_type_resolver.resolve(alternative.type_())? {
                Type::Any(_) => None,
                Type::Boolean(_)
                | Type::Function(_)
                | Type::List(_)
                | Type::None(_)
                | Type::Number(_)
                | Type::Record(_)
                | Type::String(_) => Some(vec![ssf::ir::AlgebraicAlternative::new(
                    ssf::ir::Constructor::new(
                        argument_type.clone(),
                        self.union_tag_calculator.calculate(alternative.type_())?,
                    ),
                    vec![variable_name.into()],
                    self.compile(alternative.expression())?,
                )]),
                Type::Union(union_type) => {
                    let alternative_type = self.type_compiler.compile_union(&union_type)?;

                    Some(
                        union_type
                            .types()
                            .iter()
                            .map(|type_| -> Result<_, CompileError> {
                                Ok(ssf::ir::AlgebraicAlternative::new(
                                    ssf::ir::Constructor::new(
                                        argument_type.clone(),
                                        self.union_tag_calculator.calculate(type_)?,
                                    ),
                                    vec![variable_name.into()],
                                    ssf::ir::Let::new(
                                        variable_name,
                                        alternative_type.clone(),
                                        ssf::ir::ConstructorApplication::new(
                                            ssf::ir::Constructor::new(
                                                alternative_type.clone(),
                                                self.union_tag_calculator.calculate(type_)?,
                                            ),
                                            vec![ssf::ir::Variable::new(variable_name).into()],
                                        ),
                                        self.compile(alternative.expression())?,
                                    ),
                                ))
                            })
                            .collect::<Result<Vec<_>, _>>()?,
                    )
                }
                Type::Reference(_) | Type::Unknown(_) | Type::Variable(_) => {
                    unreachable!()
                }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::error_type_configuration::ERROR_TYPE_CONFIGURATION;
    use super::super::list_type_configuration::LIST_TYPE_CONFIGURATION;
    use super::super::string_type_configuration::STRING_TYPE_CONFIGURATION;
    use super::super::type_canonicalizer::TypeCanonicalizer;
    use super::super::type_comparability_checker::TypeComparabilityChecker;
    use super::super::type_equality_checker::TypeEqualityChecker;
    use super::*;
    use crate::debug::SourceInformation;
    use pretty_assertions::assert_eq;

    fn create_expression_compiler(
        module: &Module,
    ) -> (
        Arc<ExpressionCompiler>,
        Arc<TypeCompiler>,
        Arc<UnionTagCalculator>,
    ) {
        let reference_type_resolver = ReferenceTypeResolver::new(&module);
        let last_result_type_calculator =
            LastResultTypeCalculator::new(reference_type_resolver.clone());
        let union_tag_calculator = UnionTagCalculator::new(reference_type_resolver.clone());
        let type_compiler = TypeCompiler::new(
            reference_type_resolver.clone(),
            union_tag_calculator.clone(),
            LIST_TYPE_CONFIGURATION.clone(),
        );
        let boolean_compiler = BooleanCompiler::new(type_compiler.clone());
        let none_compiler = NoneCompiler::new(type_compiler.clone());
        let variable_compiler = VariableCompiler::new(
            type_compiler.clone(),
            reference_type_resolver.clone(),
            &module,
        )
        .unwrap();
        let type_comparability_checker =
            TypeComparabilityChecker::new(reference_type_resolver.clone());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
        let type_canonicalizer = TypeCanonicalizer::new(
            reference_type_resolver.clone(),
            type_equality_checker.clone(),
        );
        let equal_operation_transformer = EqualOperationTransformer::new(
            reference_type_resolver.clone(),
            type_comparability_checker,
            type_equality_checker.clone(),
            LIST_TYPE_CONFIGURATION.clone(),
        );
        let not_equal_operation_transformer = NotEqualOperationTransformer::new();
        let list_literal_transformer = ListLiteralTransformer::new(
            reference_type_resolver.clone(),
            LIST_TYPE_CONFIGURATION.clone(),
        );
        let boolean_operation_transformer = BooleanOperationTransformer::new();
        let function_type_coercion_transformer = FunctionTypeCoercionTransformer::new(
            type_equality_checker.clone(),
            reference_type_resolver.clone(),
        );
        let list_case_transformer = ListCaseTransformer::new(
            reference_type_resolver.clone(),
            LIST_TYPE_CONFIGURATION.clone(),
        );
        let let_error_transformer = LetErrorTransformer::new(
            reference_type_resolver.clone(),
            type_equality_checker,
            type_canonicalizer,
            ERROR_TYPE_CONFIGURATION.clone(),
        );

        (
            ExpressionCompiler::new(
                ExpressionCompilerSet {
                    boolean_compiler,
                    none_compiler,
                    variable_compiler,
                }
                .into(),
                ExpressionTransformerSet {
                    equal_operation_transformer,
                    not_equal_operation_transformer,
                    list_literal_transformer,
                    boolean_operation_transformer,
                    function_type_coercion_transformer,
                    list_case_transformer,
                    let_error_transformer,
                }
                .into(),
                reference_type_resolver,
                last_result_type_calculator,
                union_tag_calculator.clone(),
                type_compiler.clone(),
                STRING_TYPE_CONFIGURATION.clone(),
            ),
            type_compiler,
            union_tag_calculator,
        )
    }

    mod operation {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn compile_arithmetic_operation() {
            let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

            assert_eq!(
                expression_compiler.compile(
                    &ArithmeticOperation::new(
                        ArithmeticOperator::Add,
                        Number::new(1.0, SourceInformation::dummy()),
                        Number::new(2.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into(),
                ),
                Ok(
                    ssf::ir::PrimitiveOperation::new(ssf::ir::PrimitiveOperator::Add, 1.0, 2.0)
                        .into()
                )
            );
        }

        #[test]
        fn compile_number_comparison_operation() {
            let (expression_compiler, type_compiler, _) =
                create_expression_compiler(&Module::dummy());

            assert_eq!(
                expression_compiler.compile(
                    &OrderOperation::new(
                        OrderOperator::LessThan,
                        Number::new(1.0, SourceInformation::dummy()),
                        Number::new(2.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into(),
                ),
                Ok(ssf::ir::PrimitiveCase::new(
                    ssf::ir::PrimitiveOperation::new(
                        ssf::ir::PrimitiveOperator::LessThan,
                        1.0,
                        2.0
                    ),
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

        #[test]
        fn compile_pipe_operation() {
            let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

            assert_eq!(
                expression_compiler.compile(
                    &PipeOperation::new(
                        Number::new(1.0, SourceInformation::dummy()),
                        Variable::new("f", SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into(),
                ),
                Ok(ssf::ir::FunctionApplication::new(ssf::ir::Variable::new("f"), 1.0).into())
            );
        }
    }

    #[test]
    fn compile_let() {
        let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

        assert_eq!(
            expression_compiler.compile(
                &Let::new(
                    vec![VariableDefinition::new(
                        "x",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )],
                    Variable::new("x", SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into(),
            ),
            Ok(ssf::ir::Let::new(
                "x",
                ssf::types::Primitive::Float64,
                42.0,
                ssf::ir::Variable::new("x")
            )
            .into())
        );
    }

    #[test]
    fn compile_let_with_multiple_definitions() {
        let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

        assert_eq!(
            expression_compiler.compile(
                &Let::new(
                    vec![
                        VariableDefinition::new(
                            "x",
                            Number::new(42.0, SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        VariableDefinition::new(
                            "y",
                            Number::new(42.0, SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        )
                    ],
                    Variable::new("x", SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into(),
            ),
            Ok(ssf::ir::Let::new(
                "x",
                ssf::types::Primitive::Float64,
                42.0,
                ssf::ir::Let::new(
                    "y",
                    ssf::types::Primitive::Float64,
                    42.0,
                    ssf::ir::Variable::new("x")
                )
            )
            .into())
        );
    }

    #[test]
    fn compile_let_recursive() {
        let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

        assert_eq!(
            expression_compiler.compile(
                &LetRecursive::new(
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
                    )],
                    Variable::new("x", SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into(),
            ),
            Ok(ssf::ir::LetRecursive::new(
                vec![ssf::ir::Definition::new(
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
    fn compile_let_recursive_with_recursive_functions() {
        let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

        assert_eq!(
            expression_compiler.compile(
                &LetRecursive::new(
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
                    )],
                    Variable::new("x", SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into(),
            ),
            Ok(ssf::ir::LetRecursive::new(
                vec![ssf::ir::Definition::new(
                    "f",
                    vec![ssf::ir::Argument::new("x", ssf::types::Primitive::Float64)],
                    ssf::ir::FunctionApplication::new(
                        ssf::ir::Variable::new("f"),
                        ssf::ir::Variable::new("x"),
                    ),
                    ssf::types::Primitive::Float64,
                )],
                ssf::ir::Variable::new("x")
            )
            .into())
        );
    }

    #[test]
    fn compile_nested_let_recursive() {
        let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

        assert_eq!(
            expression_compiler.compile(
                &LetRecursive::new(
                    vec![FunctionDefinition::new(
                        "f",
                        vec!["x".into()],
                        LetRecursive::new(
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
                            )],
                            Variable::new("x", SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        SourceInformation::dummy()
                    )],
                    Variable::new("x", SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into(),
            ),
            Ok(ssf::ir::LetRecursive::new(
                vec![ssf::ir::Definition::new(
                    "f",
                    vec![ssf::ir::Argument::new("x", ssf::types::Primitive::Float64)],
                    ssf::ir::LetRecursive::new(
                        vec![ssf::ir::Definition::new(
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
    fn compile_let_with_free_variables() {
        let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

        assert_eq!(
            expression_compiler.compile(
                &Let::new(
                    vec![VariableDefinition::new(
                        "y",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )],
                    LetRecursive::new(
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
                        )],
                        Variable::new("y", SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                )
                .into(),
            ),
            Ok(ssf::ir::Let::new(
                "y",
                ssf::types::Primitive::Float64,
                42.0,
                ssf::ir::LetRecursive::new(
                    vec![ssf::ir::Definition::new(
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
    fn compile_case_expression_with_any_type_argument() {
        let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

        insta::assert_debug_snapshot!(expression_compiler.compile(
            &Case::with_type(
                types::Any::new(SourceInformation::dummy()),
                "x",
                Let::new(
                    vec![VariableDefinition::new(
                        "y",
                        None::new(SourceInformation::dummy()),
                        types::Any::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )],
                    Variable::new("y", SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                vec![
                    Alternative::new(
                        types::None::new(SourceInformation::dummy()),
                        None::new(SourceInformation::dummy())
                    ),
                    Alternative::new(
                        types::Any::new(SourceInformation::dummy()),
                        None::new(SourceInformation::dummy())
                    )
                ],
                SourceInformation::dummy(),
            )
            .into(),
        ));
    }

    #[test]
    fn fail_to_compile_case_expression_with_argument_type_invalid() {
        let (expression_compiler, _, _) = create_expression_compiler(&Module::dummy());

        assert_eq!(
            expression_compiler.compile(
                &Case::with_type(
                    types::Boolean::new(SourceInformation::dummy()),
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
                vec![TypeDefinition::new("Foo", type_)],
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
                        lower_union_type,
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
                        .into_algebraic()
                        .unwrap(),
                )
                .into())
            );
        }
    }
}
