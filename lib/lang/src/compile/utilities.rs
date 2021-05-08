use super::error::CompileError;
use crate::types::{self, Type};

pub fn get_record_element<'a>(
    record_type: &'a types::Record,
    name: &str,
) -> Result<&'a Type, CompileError> {
    record_type
        .elements()
        .iter()
        .find(|element| element.name() == name)
        .ok_or_else(|| CompileError::RecordElementNotFound {
            record_type: record_type.clone(),
            name: name.into(),
        })
        .map(|element| element.type_())
}
