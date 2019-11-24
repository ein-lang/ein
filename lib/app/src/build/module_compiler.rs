use super::relative_module_path_converter::RelativeModulePathConverter;
use crate::infra::{FilePath, FileStorage};

pub struct ModuleCompiler<'a, S: FileStorage> {
    relative_module_path_converter: &'a RelativeModulePathConverter<'a>,
    object_file_storage: &'a S,
    interface_file_storage: &'a S,
}

impl<'a, S: FileStorage> ModuleCompiler<'a, S> {
    pub fn new(
        relative_module_path_converter: &'a RelativeModulePathConverter,
        object_file_storage: &'a S,
        interface_file_storage: &'a S,
    ) -> Self {
        Self {
            relative_module_path_converter,
            object_file_storage,
            interface_file_storage,
        }
    }

    pub fn compile(
        &self,
        file_path: &FilePath,
        // Callers pass unresolved modules to omit extra reading and parsing of
        // source files.
        module: &ein::ast::UnresolvedModule,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (module_object, module_interface) = ein::compile(
            &module.clone().resolve(
                self.relative_module_path_converter
                    .convert_from_file_path(file_path),
                module
                    .imports()
                    .iter()
                    .map(|import| match import.module_path() {
                        ein::UnresolvedModulePath::Absolute(_) => unimplemented!(),
                        ein::UnresolvedModulePath::Relative(relative_module_path) => {
                            Ok(ein::deserialize_module_interface(
                                &self.interface_file_storage.read_to_vec(
                                    &self
                                        .relative_module_path_converter
                                        .convert_to_file_path(relative_module_path),
                                )?,
                            )?)
                        }
                    })
                    .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?,
            ),
        )?;

        self.object_file_storage
            .write(&file_path, &module_object.serialize())?;
        self.interface_file_storage.write(
            &file_path,
            &ein::serialize_module_interface(&module_interface)?,
        )?;

        Ok(())
    }
}
