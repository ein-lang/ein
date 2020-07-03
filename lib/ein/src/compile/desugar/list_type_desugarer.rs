use super::super::error::CompileError;
use super::super::list_literal_configuration::ListLiteralConfiguration;
use crate::ast::*;
use crate::types::{self, Type};
use std::sync::Arc;

pub struct ListTypeDesugarer {
    configuration: Arc<ListLiteralConfiguration>,
}

impl ListTypeDesugarer {
    pub fn new(configuration: Arc<ListLiteralConfiguration>) -> Self {
        Self { configuration }
    }

    pub fn desugar(&mut self, module: &Module) -> Result<Module, CompileError> {
        module.convert_types(&mut |type_| -> Result<Type, CompileError> {
            Ok(self.desugar_type(type_))
        })
    }

    fn desugar_type(&mut self, type_: &Type) -> Type {
        if let Type::List(list) = type_ {
            types::Reference::new(
                self.configuration.list_type_name(),
                list.source_information().clone(),
            )
            .into()
        } else {
            type_.clone()
        }
    }
}
