#[derive(Clone, Debug, PartialEq)]
pub enum ModulePath {
    External(Vec<String>),
    Internal(Vec<String>),
}
