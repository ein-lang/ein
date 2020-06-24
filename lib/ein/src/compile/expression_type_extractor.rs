use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::union_type_simplifier::UnionTypeSimplifier;
use crate::ast::*;
use crate::types::{self, Type};
use std::collections::HashMap;
use std::rc::Rc;

pub struct ExpressionTypeExtractor {
    reference_type_resolver: Rc<ReferenceTypeResolver>,
    union_type_simplifier: Rc<UnionTypeSimplifier>,
}

impl ExpressionTypeExtractor {
    pub fn new(
        reference_type_resolver: Rc<ReferenceTypeResolver>,
        union_type_simplifier: Rc<UnionTypeSimplifier>,
    ) -> Rc<Self> {
        Self {
            reference_type_resolver,
            union_type_simplifier,
        }
        .into()
    }

    pub fn extract(
        &self,
        expression: &Expression,
        variables: &HashMap<String, Type>,
    ) -> Result<Type, CompileError> {
        Ok(match expression {
            Expression::Application(application) => self
                .extract(application.function(), variables)?
                .to_function()
                .unwrap()
                .result()
                .clone(),
            Expression::Boolean(boolean) => {
                types::Boolean::new(boolean.source_information().clone()).into()
            }
            Expression::Case(case) => {
                self.union_type_simplifier
                    .simplify_union(&types::Union::new(
                        case.alternatives()
                            .iter()
                            .map(|alternative| {
                                let mut variables = variables.clone();

                                variables.insert(case.name().into(), alternative.type_().clone());

                                self.extract(alternative.expression(), &variables)
                            })
                            .collect::<Result<_, _>>()?,
                        case.source_information().clone(),
                    ))?
            }
            Expression::If(if_) => {
                self.union_type_simplifier
                    .simplify_union(&types::Union::new(
                        vec![
                            self.extract(if_.then(), variables)?,
                            self.extract(if_.else_(), variables)?,
                        ],
                        if_.source_information().clone(),
                    ))?
            }
            Expression::Let(let_) => {
                let mut variables = variables.clone();

                for definition in let_.definitions() {
                    match definition {
                        Definition::FunctionDefinition(function_definition) => {
                            variables.insert(
                                function_definition.name().into(),
                                function_definition.type_().clone(),
                            );
                        }
                        Definition::ValueDefinition(value_definition) => {
                            variables.insert(
                                value_definition.name().into(),
                                value_definition.type_().clone(),
                            );
                        }
                    }
                }

                self.extract(let_.expression(), &variables)?
            }
            Expression::None(none) => types::None::new(none.source_information().clone()).into(),
            Expression::Number(number) => {
                types::Number::new(number.source_information().clone()).into()
            }
            Expression::Operation(operation) => match operation.operator() {
                Operator::Add | Operator::Subtract | Operator::Multiply | Operator::Divide => {
                    types::Number::new(operation.source_information().clone()).into()
                }
                Operator::Equal
                | Operator::NotEqual
                | Operator::LessThan
                | Operator::LessThanOrEqual
                | Operator::GreaterThan
                | Operator::GreaterThanOrEqual
                | Operator::And
                | Operator::Or => {
                    types::Boolean::new(operation.source_information().clone()).into()
                }
            },
            Expression::RecordConstruction(record) => record.type_().clone(),
            Expression::RecordElementOperation(operation) => {
                let type_ = self.reference_type_resolver.resolve(operation.type_())?;

                type_.to_record().unwrap().elements()[operation.key()].clone()
            }
            Expression::TypeCoercion(coercion) => coercion.to().clone(),
            Expression::Variable(variable) => variables[variable.name()].clone(),
            Expression::RecordUpdate(_) => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::reference_type_resolver::ReferenceTypeResolver;
    use super::super::type_equality_checker::TypeEqualityChecker;
    use super::*;
    use crate::debug::SourceInformation;

    #[test]
    fn extract_type_of_case_expression() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
        let union_type_simplifier = UnionTypeSimplifier::new(
            reference_type_resolver.clone(),
            type_equality_checker.clone(),
        );

        assert_eq!(
            ExpressionTypeExtractor::new(reference_type_resolver, union_type_simplifier).extract(
                &Case::new(
                    "",
                    None::new(SourceInformation::dummy()),
                    vec![
                        Alternative::new(
                            types::Boolean::new(SourceInformation::dummy()),
                            Boolean::new(false, SourceInformation::dummy()),
                        ),
                        Alternative::new(
                            types::None::new(SourceInformation::dummy()),
                            None::new(SourceInformation::dummy()),
                        )
                    ],
                    SourceInformation::dummy()
                )
                .into(),
                &Default::default(),
            ),
            Ok(types::Union::new(
                vec![
                    types::Boolean::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into()
                ],
                SourceInformation::dummy()
            )
            .into())
        );
    }
}
