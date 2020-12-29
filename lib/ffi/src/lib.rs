mod allocator;
mod boolean;
mod closure;
mod none;
mod number;
mod string;

#[cfg(not(test))]
pub use allocator::initialize;
pub use boolean::*;
pub use closure::*;
pub use none::*;
pub use number::*;
pub use string::*;
