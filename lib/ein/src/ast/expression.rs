use super::application::Application;
use super::boolean::Boolean;
use super::if_::If;
use super::let_::Let;
use super::none::None;
use super::number::Number;
use super::operation::Operation;
use super::record::Record;
use super::record_update::RecordUpdate;
use super::variable::Variable;
use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Application(Application),
    Boolean(Boolean),
    Record(Record),
    RecordUpdate(RecordUpdate),
    If(If),
    Let(Let),
    None(None),
    Number(Number),
    Operation(Operation),
    Variable(Variable),
}

impl Expression {
    pub fn convert_expressions(&self, convert: &mut impl FnMut(&Expression) -> Expression) -> Self {
        let expression = match self {
            Self::Application(application) => application.convert_expressions(convert).into(),
            Self::Record(record) => record.convert_expressions(convert).into(),
            Self::RecordUpdate(record_update) => record_update.convert_expressions(convert).into(),
            Self::If(if_) => if_.convert_expressions(convert).into(),
            Self::Let(let_) => let_.convert_expressions(convert).into(),
            Self::Operation(operation) => operation.convert_expressions(convert).into(),
            Self::Boolean(_) | Self::None(_) | Self::Number(_) | Self::Variable(_) => self.clone(),
        };

        convert(&expression)
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Type) -> Type) -> Self {
        match self {
            Self::Application(application) => application.convert_types(convert).into(),
            Self::Record(record) => record.convert_types(convert).into(),
            Self::RecordUpdate(record_update) => record_update.convert_types(convert).into(),
            Self::If(if_) => if_.convert_types(convert).into(),
            Self::Let(let_) => let_.convert_types(convert).into(),
            Self::Operation(operation) => operation.convert_types(convert).into(),
            Self::Boolean(_) | Self::None(_) | Self::Number(_) | Self::Variable(_) => self.clone(),
        }
    }
}

impl From<Application> for Expression {
    fn from(application: Application) -> Expression {
        Self::Application(application)
    }
}

impl From<Boolean> for Expression {
    fn from(boolean: Boolean) -> Expression {
        Self::Boolean(boolean)
    }
}

impl From<Record> for Expression {
    fn from(record: Record) -> Expression {
        Self::Record(record)
    }
}

impl From<RecordUpdate> for Expression {
    fn from(record_update: RecordUpdate) -> Expression {
        Self::RecordUpdate(record_update)
    }
}

impl From<If> for Expression {
    fn from(if_: If) -> Expression {
        Self::If(if_)
    }
}

impl From<Let> for Expression {
    fn from(let_: Let) -> Expression {
        Self::Let(let_)
    }
}

impl From<None> for Expression {
    fn from(none: None) -> Expression {
        Self::None(none)
    }
}

impl From<Number> for Expression {
    fn from(number: Number) -> Expression {
        Self::Number(number)
    }
}

impl From<Operation> for Expression {
    fn from(operation: Operation) -> Expression {
        Self::Operation(operation)
    }
}

impl From<Variable> for Expression {
    fn from(variable: Variable) -> Expression {
        Self::Variable(variable)
    }
}
