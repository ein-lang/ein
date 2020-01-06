use super::function::Function;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Value {
    Number,
}

impl Value {
    pub fn to_id(&self) -> String {
        match self {
            Self::Number => "Number".into(),
        }
    }

    pub fn unwrap_once(&self, _: usize, _: &Function) -> Self {
        // TODO Unwrap map types.
        self.clone()
    }
}
