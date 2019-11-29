use super::path::ExternalModulePathManager;
use super::path::InternalModulePathManager;
use crate::infra::{FilePath, FileStorage};

pub struct ModuleCompiler<'a, S: FileStorage> {
    internal_module_path_converter: &'a InternalModulePathManager<'a>,
    external_module_path_converter: &'a ExternalModulePathManager<'a>,
    file_storage: &'a S,
}

impl<'a, S: FileStorage> ModuleCompiler<'a, S> {
    pub fn new(
        internal_module_path_converter: &'a InternalModulePathManager<'a>,
        external_module_path_converter: &'a ExternalModulePathManager<'a>,
        file_storage: &'a S,
    ) -> Self {
        Self {
            internal_module_path_converter,
            external_module_path_converter,
            file_storage,
        }
    }

    pub fn compile(
        &self,
        package: &ein::Package,
        source_file_path: &FilePath,
        object_file_path: &FilePath,
        interface_file_path: &FilePath,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let module_path = self
            .internal_module_path_converter
            .convert_to_module_path(source_file_path, package);
        let module = ein::parse_module(ein::Source::new(
            &format!("{}", source_file_path),
            &self.file_storage.read_to_string(source_file_path)?,
        ))?;

        let (module_object, module_interface) = ein::compile(
            &module.clone().resolve(
                module_path.clone(),
                module
                    .imports()
                    .iter()
                    .map(|import| match import.module_path() {
                        ein::UnresolvedModulePath::External(external_module_path) => {
                            Ok(ein::deserialize_module_interface(
                                &self.file_storage.read_to_vec(
                                    &self
                                        .external_module_path_converter
                                        .resolve_to_interface_file_path(external_module_path),
                                )?,
                            )?)
                        }
                        ein::UnresolvedModulePath::Internal(internal_module_path) => {
                            Ok(ein::deserialize_module_interface(
                                &self.file_storage.read_to_vec(
                                    &self
                                        .internal_module_path_converter
                                        .resolve_to_interface_file_path(internal_module_path),
                                )?,
                            )?)
                        }
                    })
                    .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?,
            ),
        )?;

        self.file_storage
            .write(&object_file_path, &module_object.serialize())?;
        self.file_storage.write(
            &interface_file_path,
            &ein::serialize_module_interface(&module_interface)?,
        )?;

        Ok(())
    }
}
