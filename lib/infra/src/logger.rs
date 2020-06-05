pub struct Logger {}

#[derive(Default)]
impl Logger {
    pub fn new() -> Self {
        Self {}
    }
}

impl app::Logger for Logger {
    fn log(&self, log: &str) {
        println!("{}", log);
    }
}
