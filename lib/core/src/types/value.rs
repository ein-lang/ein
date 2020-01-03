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
}
