use super::{
    definition::Definition, export::Export, export_foreign::ExportForeign, expression::Expression,
    import::Import, import_foreign::ImportForeign, type_definition::TypeDefinition,
};
use crate::{path::ModulePath, types::Type};

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    path: ModulePath,
    type_definitions: Vec<TypeDefinition>,
    definitions: Vec<Definition>,
    export: Export,
    export_foreign: ExportForeign,
    imports: Vec<Import>,
    import_foreigns: Vec<ImportForeign>,
}

impl Module {
    pub fn new(
        path: ModulePath,
        export: Export,
        export_foreign: ExportForeign,
        imports: Vec<Import>,
        import_foreigns: Vec<ImportForeign>,
        type_definitions: Vec<TypeDefinition>,
        definitions: Vec<Definition>,
    ) -> Self {
        Self {
            path,
            type_definitions,
            definitions,
            export,
            export_foreign,
            imports,
            import_foreigns,
        }
    }

    #[cfg(test)]
    pub fn dummy() -> Self {
        Self::new(
            ModulePath::new(crate::package::Package::new("", ""), vec![]),
            Export::new(Default::default()),
            ExportForeign::new(Default::default()),
            vec![],
            vec![],
            vec![],
            vec![],
        )
    }

    #[cfg(test)]
    pub fn from_definitions(definitions: Vec<Definition>) -> Self {
        Self::new(
            ModulePath::new(crate::package::Package::new("", ""), vec![]),
            Export::new(Default::default()),
            ExportForeign::new(Default::default()),
            vec![],
            vec![],
            vec![],
            definitions,
        )
    }

    #[cfg(test)]
    pub fn from_definitions_and_type_definitions(
        type_definitions: Vec<TypeDefinition>,
        definitions: Vec<Definition>,
    ) -> Self {
        Self::new(
            ModulePath::new(crate::package::Package::new("", ""), vec![]),
            Export::new(Default::default()),
            ExportForeign::new(Default::default()),
            vec![],
            vec![],
            type_definitions,
            definitions,
        )
    }

    pub fn path(&self) -> &ModulePath {
        &self.path
    }

    pub fn type_definitions(&self) -> &[TypeDefinition] {
        &self.type_definitions
    }

    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }

    pub fn export(&self) -> &Export {
        &self.export
    }

    pub fn export_foreign(&self) -> &ExportForeign {
        &self.export_foreign
    }

    pub fn imports(&self) -> &[Import] {
        &self.imports
    }

    pub fn import_foreigns(&self) -> &[ImportForeign] {
        &self.import_foreigns
    }

    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.path.clone(),
            self.export.clone(),
            self.export_foreign.clone(),
            self.imports.clone(),
            self.import_foreigns.clone(),
            self.type_definitions.clone(),
            self.definitions
                .iter()
                .map(|definition| definition.transform_expressions(transform))
                .collect::<Result<_, _>>()?,
        ))
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(Self::new(
            self.path.clone(),
            self.export.clone(),
            self.export_foreign.clone(),
            self.imports.clone(),
            self.import_foreigns
                .iter()
                .map(|declaration| declaration.transform_types(transform))
                .collect::<Result<_, _>>()?,
            self.type_definitions
                .iter()
                .map(|type_definition| type_definition.transform_types(transform))
                .collect::<Result<_, _>>()?,
            self.definitions
                .iter()
                .map(|definition| definition.transform_types(transform))
                .collect::<Result<_, _>>()?,
        ))
    }
}
