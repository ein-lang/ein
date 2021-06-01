use super::super::{
    error::CompileError, name_generator::NameGenerator,
    reference_type_resolver::ReferenceTypeResolver, type_equality_checker::TypeEqualityChecker,
};
use crate::{ast::*, debug::SourceInformation, types::Type};
use std::sync::Arc;

pub struct FunctionTypeCoercionTransformer {
    function_name_generator: NameGenerator,
    argument_name_generator: NameGenerator,
    type_equality_checker: Arc<TypeEqualityChecker>,
    reference_type_resolver: Arc<ReferenceTypeResolver>,
}

impl FunctionTypeCoercionTransformer {
    pub fn new(
        type_equality_checker: Arc<TypeEqualityChecker>,
        reference_type_resolver: Arc<ReferenceTypeResolver>,
    ) -> Arc<Self> {
        Self {
            function_name_generator: NameGenerator::new("$tc_func_"),
            argument_name_generator: NameGenerator::new("$tc_arg_"),
            type_equality_checker,
            reference_type_resolver,
        }
        .into()
    }

    pub fn transform(&self, coercion: &TypeCoercion) -> Result<Expression, CompileError> {
        Ok(match self.reference_type_resolver.resolve(coercion.to())? {
            Type::Any(_) | Type::Union(_) => coercion.clone().into(),
            Type::Function(_) => {
                let source_information = coercion.source_information();
                let function_name = self.function_name_generator.generate();

                Let::new(
                    vec![VariableDefinition::new(
                        function_name.clone(),
                        coercion.argument().clone(),
                        coercion.from().clone(),
                        source_information.clone(),
                    )
                    .into()],
                    self.transform_function(
                        Variable::new(function_name, source_information.clone()).into(),
                        coercion.from(),
                        coercion.to(),
                        source_information.clone(),
                    )?,
                    source_information.clone(),
                )
                .into()
            }
            _ => unreachable!(),
        })
    }

    fn transform_function(
        &self,
        argument: Expression,
        from_type: &Type,
        to_type: &Type,
        source_information: Arc<SourceInformation>,
    ) -> Result<Expression, CompileError> {
        Ok(if self.type_equality_checker.equal(from_type, to_type)? {
            argument
        } else {
            match (
                self.reference_type_resolver.resolve(from_type)?,
                self.reference_type_resolver.resolve(to_type)?,
            ) {
                (Type::Function(from_function_type), Type::Function(to_function_type)) => {
                    let from_function_name = self.function_name_generator.generate();
                    let to_function_name = self.function_name_generator.generate();
                    let argument_name = self.argument_name_generator.generate();

                    // Curry functions to preserve their evaluation orders.
                    Let::new(
                        vec![FunctionDefinition::new(
                            to_function_name.clone(),
                            vec![argument_name.clone()],
                            Let::new(
                                vec![VariableDefinition::new(
                                    from_function_name.clone(),
                                    Application::with_type(
                                        from_function_type.clone(),
                                        argument,
                                        self.coerce_type(
                                            Variable::new(
                                                argument_name,
                                                source_information.clone(),
                                            ),
                                            to_function_type.argument(),
                                            from_function_type.argument(),
                                            source_information.clone(),
                                        )?,
                                        source_information.clone(),
                                    ),
                                    from_function_type.result().clone(),
                                    source_information.clone(),
                                )
                                .into()],
                                self.transform_function(
                                    Variable::new(from_function_name, source_information.clone())
                                        .into(),
                                    from_function_type.result(),
                                    to_function_type.result(),
                                    source_information.clone(),
                                )?,
                                source_information.clone(),
                            ),
                            to_function_type.clone(),
                            source_information.clone(),
                        )
                        .into()],
                        Variable::new(to_function_name, source_information.clone()),
                        source_information,
                    )
                    .into()
                }
                (_, Type::Function(_)) => unreachable!(),
                _ => self.coerce_type(argument, from_type, to_type, source_information)?,
            }
        })
    }

    fn coerce_type(
        &self,
        argument: impl Into<Expression>,
        from_type: &Type,
        to_type: &Type,
        source_information: Arc<SourceInformation>,
    ) -> Result<Expression, CompileError> {
        Ok(if self.type_equality_checker.equal(from_type, to_type)? {
            argument.into()
        } else {
            TypeCoercion::new(
                argument,
                from_type.clone(),
                to_type.clone(),
                source_information,
            )
            .into()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{debug::SourceInformation, types};
    use pretty_assertions::assert_eq;

    fn create_function_type_coercion_transformer() -> Arc<FunctionTypeCoercionTransformer> {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());

        FunctionTypeCoercionTransformer::new(type_equality_checker, reference_type_resolver)
    }

    #[test]
    fn transform() {
        let from_type = types::Function::new(
            types::Any::new(SourceInformation::dummy()),
            types::Number::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        );
        let to_type = types::Function::new(
            types::Number::new(SourceInformation::dummy()),
            types::Any::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        );

        assert_eq!(
            create_function_type_coercion_transformer().transform(&TypeCoercion::new(
                Variable::new("f", SourceInformation::dummy()),
                from_type.clone(),
                to_type.clone(),
                SourceInformation::dummy(),
            )),
            Ok(Let::new(
                vec![VariableDefinition::new(
                    "$tc_func_0",
                    Variable::new("f", SourceInformation::dummy()),
                    from_type.clone(),
                    SourceInformation::dummy()
                )
                .into()],
                Let::new(
                    vec![FunctionDefinition::new(
                        "$tc_func_2",
                        vec!["$tc_arg_0".into()],
                        Let::new(
                            vec![VariableDefinition::new(
                                "$tc_func_1",
                                Application::with_type(
                                    from_type.clone(),
                                    Variable::new("$tc_func_0", SourceInformation::dummy()),
                                    TypeCoercion::new(
                                        Variable::new("$tc_arg_0", SourceInformation::dummy()),
                                        to_type.argument().clone(),
                                        from_type.argument().clone(),
                                        SourceInformation::dummy()
                                    ),
                                    SourceInformation::dummy()
                                ),
                                from_type.result().clone(),
                                SourceInformation::dummy()
                            )
                            .into()],
                            TypeCoercion::new(
                                Variable::new("$tc_func_1", SourceInformation::dummy()),
                                from_type.result().clone(),
                                to_type.result().clone(),
                                SourceInformation::dummy(),
                            ),
                            SourceInformation::dummy(),
                        ),
                        to_type.clone(),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("$tc_func_2", SourceInformation::dummy()),
                    SourceInformation::dummy(),
                ),
                SourceInformation::dummy(),
            )
            .into()),
        );
    }
}
