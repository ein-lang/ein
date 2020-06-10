use crate::ast::*;
use crate::parse::*;
use indoc::indoc;

pub struct BuiltinFunctionCreator {}

impl BuiltinFunctionCreator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create(&mut self, module: &Module) -> Module {
        let prelude_module = parse(
            indoc!(
                "
                export { not }

                not : Boolean -> Boolean
                not x = if x then false else true
                "
            ),
            "<prelude>",
        )
        .unwrap();

        Module::new(
            module.path().clone(),
            module.export().clone(),
            module.imports().to_vec(),
            module.type_definitions().to_vec(),
            module
                .definitions()
                .iter()
                .cloned()
                .chain(prelude_module.definitions().to_vec())
                .collect(),
        )
    }
}
