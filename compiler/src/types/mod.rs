mod function;

pub use function::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Function(Function),
    Number,
}
