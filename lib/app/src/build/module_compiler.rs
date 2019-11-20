use super::relative_module_path_converter::RelativeModulePathConverter;
use crate::infra::{FilePath, FileStorage};
use std::hash::{Hash, Hasher};

pub struct ModuleCompiler<'a, S: FileStorage> {
    relative_module_path_converter: &'a RelativeModulePathConverter<'a>,
    source_file_storage: &'a S,
    object_file_storage: &'a S,
    interface_file_storage: &'a S,
}

impl<'a, S: FileStorage> ModuleCompiler<'a, S> {
    pub fn new(
        relative_module_path_converter: &'a RelativeModulePathConverter,
        source_file_storage: &'a S,
        object_file_storage: &'a S,
        interface_file_storage: &'a S,
    ) -> Self {
        Self {
            relative_module_path_converter,
            source_file_storage,
            object_file_storage,
            interface_file_storage,
        }
    }

    pub fn compile(
        &self,
        source_file_path: &FilePath,
    ) -> Result<FilePath, Box<dyn std::error::Error>> {
        let source = self.source_file_storage.read_to_string(source_file_path)?;
        let module =
            ein::parse_module(ein::Source::new(&format!("{}", source_file_path), &source))?;

        let imported_target_file_paths = module
            .imports()
            .iter()
            .map(|import| match import.module_path() {
                ein::UnresolvedModulePath::Absolute(_) => unimplemented!(),
                ein::UnresolvedModulePath::Relative(relative_module_path) => self.compile(
                    &self
                        .relative_module_path_converter
                        .convert_to_file_path(relative_module_path),
                ),
            })
            .collect::<Result<Vec<_>, _>>()?;

        let target_file_path = Self::calculate_target_file_path(
            source_file_path,
            &source,
            &imported_target_file_paths,
        );

        if self.object_file_storage.exists(&target_file_path)
            && self.interface_file_storage.exists(&target_file_path)
        {
            return Ok(target_file_path);
        }

        let (mut module_object, module_interface) = ein::compile(
            &module.resolve(
                self.relative_module_path_converter
                    .convert_from_file_path(source_file_path),
                imported_target_file_paths
                    .iter()
                    .map(|file_path| {
                        Ok(ein::deserialize_module_interface(
                            &self.interface_file_storage.read_to_vec(file_path)?,
                        )?)
                    })
                    .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?,
            ),
        )?;

        for file_path in &imported_target_file_paths {
            module_object.link(ein::ModuleObject::deserialize(
                &self.object_file_storage.read_to_vec(file_path)?,
            ));
        }

        self.object_file_storage
            .write(&target_file_path, &module_object.serialize())?;
        self.interface_file_storage.write(
            &target_file_path,
            &ein::serialize_module_interface(&module_interface)?,
        )?;

        Ok(target_file_path)
    }

    fn calculate_target_file_path(
        source_file_path: &FilePath,
        source: &str,
        imported_target_file_paths: &[FilePath],
    ) -> FilePath {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        source_file_path.hash(&mut hasher);
        source.hash(&mut hasher);

        for file_path in imported_target_file_paths {
            file_path.hash(&mut hasher);
        }

        FilePath::new(vec![format!("{:x}", hasher.finish())])
    }
}
