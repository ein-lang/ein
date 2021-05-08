use super::error::CompileError;
use crate::types::{self, Type};

pub fn get_record_element<'a>(
    record_type: &'a types::Record,
    element_name: &str,
) -> Result<&'a Type, CompileError> {
    record_type
        .elements()
        .iter()
        .find(|element| element.name() == element_name)
        .ok_or_else(|| CompileError::RecordElementNotFound {
            record_type: record_type.clone(),
            name: element_name.into(),
        })
        .map(|element| element.type_())
}
