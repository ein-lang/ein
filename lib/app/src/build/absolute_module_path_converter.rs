use crate::infra::FilePath;

pub struct AbsoluteModulePathConverter;

impl AbsoluteModulePathConverter {
    pub fn convert_to_file_path(
        absolute_module_path_converter: &ein::AbsoluteUnresolvedModulePath,
    ) -> FilePath {
        FilePath::new(absolute_module_path_converter.components().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_file_path() {
        assert_eq!(
            AbsoluteModulePathConverter::convert_to_file_path(
                &ein::AbsoluteUnresolvedModulePath::new(vec!["package".into(), "Foo".into()])
                    .into()
            ),
            FilePath::new(vec!["package".into(), "Foo".into()])
        );
    }
}
