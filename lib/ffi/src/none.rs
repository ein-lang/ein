#[repr(C)]
pub struct None {}

impl None {
    pub fn new() -> Self {
        Self {}
    }
}
