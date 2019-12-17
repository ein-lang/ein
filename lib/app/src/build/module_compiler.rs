use super::error::BuildError;
use super::path::FilePathManager;
use crate::infra::{FilePath, FileStorage};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub struct ModuleCompiler<'a, S: FileStorage> {
    file_path_manager: &'a FilePathManager<'a>,
    file_storage: &'a S,
}

impl<'a, S: FileStorage> ModuleCompiler<'a, S> {
    pub fn new(file_path_manager: &'a FilePathManager<'a>, file_storage: &'a S) -> Self {
        Self {
            file_path_manager,
            file_storage,
        }
    }

    pub fn compile(
        &self,
        package: &ein::Package,
        module_interfaces: &HashMap<ein::UnresolvedModulePath, ein::ast::ModuleInterface>,
        source_file_path: &FilePath,
    ) -> Result<(FilePath, FilePath), Box<dyn std::error::Error>> {
        let source = self.file_storage.read_to_string(source_file_path)?;
        let module = ein::parse_module(&source, &format!("{}", source_file_path))?;

        let imported_module_interfaces = module
            .imports()
            .iter()
            .map(|import| {
                Ok(module_interfaces
                    .get(import.module_path())
                    .ok_or(BuildError::ModuleNotFound {
                        module_path: import.module_path().clone(),
                    })?
                    .clone())
            })
            .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;

        let object_file_path =
            self.generate_object_file_path(source_file_path, &source, &imported_module_interfaces);
        let interface_file_path = object_file_path.with_extension(
            self.file_path_manager
                .configuration()
                .interface_file_extension(),
        );

        if self.file_storage.exists(&object_file_path) {
            return Ok((object_file_path, interface_file_path));
        }

        let (module_object, module_interface) = ein::compile(
            &module.resolve(
                self.file_path_manager
                    .convert_to_module_path(source_file_path, package),
                imported_module_interfaces,
            ),
        )?;

        self.file_storage
            .write(&object_file_path, &module_object.serialize())?;
        self.file_storage.write(
            &interface_file_path,
            &ein::serialize_module_interface(&module_interface)?,
        )?;

        Ok((object_file_path, interface_file_path))
    }

    fn generate_object_file_path<'b>(
        &self,
        source_file_path: &FilePath,
        source: &str,
        imported_module_interfaces: impl IntoIterator<Item = &'b ein::ast::ModuleInterface>,
    ) -> FilePath {
        let mut hasher = DefaultHasher::new();

        source_file_path.hash(&mut hasher);
        source.hash(&mut hasher);

        for module_interface in imported_module_interfaces {
            module_interface.hash(&mut hasher);
        }

        self.file_path_manager
            .configuration()
            .object_directory()
            .join(&FilePath::new(&[&format!("{:x}", hasher.finish())]))
            .with_extension(
                self.file_path_manager
                    .configuration()
                    .object_file_extension(),
            )
    }
}
