use std::collections::HashMap;

pub fn convert_module_interface_vec_to_map(
    module_interfaces: &[lang::ModuleInterface],
) -> HashMap<lang::ExternalUnresolvedModulePath, lang::ModuleInterface> {
    module_interfaces
        .iter()
        .map(|module_interface| {
            (
                module_interface.path().external_unresolved(),
                module_interface.clone(),
            )
        })
        .collect()
}
