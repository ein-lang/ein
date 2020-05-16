use super::module_interface::ModuleInterface;

#[derive(Clone, Debug, PartialEq)]
pub struct Import {
    module_interface: ModuleInterface,
    qualified: bool,
}

impl Import {
    pub fn new(module_interface: ModuleInterface, qualified: bool) -> Self {
        Self {
            module_interface,
            qualified,
        }
    }

    pub fn module_interface(&self) -> &ModuleInterface {
        &self.module_interface
    }

    pub fn qualified(&self) -> bool {
        self.qualified
    }
}
