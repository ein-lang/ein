use crate::ast::Module;
use crate::ast::ModuleInterface;

#[derive(Debug)]
pub struct ModuleInterfaceCompiler {}

impl ModuleInterfaceCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&self, module: &Module) -> ModuleInterface {
        ModuleInterface::new(
            module.path().clone(),
            module
                .definitions()
                .iter()
                .filter(|definition| module.export().names().contains(definition.name()))
                .map(|definition| (definition.name().into(), definition.type_().clone()))
                .collect(),
        )
    }
}
