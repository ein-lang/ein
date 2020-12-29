#[repr(C)]
#[derive(Default)]
pub struct None {}

impl None {
    pub fn new() -> Self {
        Self {}
    }
}
