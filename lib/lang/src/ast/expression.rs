use super::application::Application;
use super::boolean::Boolean;
use super::case::Case;
use super::if_::If;
use super::let_::Let;
use super::let_recursive::LetRecursive;
use super::list::List;
use super::list_case::ListCase;
use super::none::None;
use super::number::Number;
use super::operation::Operation;
use super::record_construction::RecordConstruction;
use super::record_element_operation::RecordElementOperation;
use super::record_update::RecordUpdate;
use super::string::EinString;
use super::type_coercion::TypeCoercion;
use super::variable::Variable;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Application(Application),
    Boolean(Boolean),
    Case(Case),
    If(If),
    Let(Let),
    LetRecursive(LetRecursive),
    List(List),
    ListCase(ListCase),
    None(None),
    Number(Number),
    Operation(Operation),
    RecordConstruction(RecordConstruction),
    RecordElementOperation(RecordElementOperation),
    RecordUpdate(RecordUpdate),
    String(EinString),
    TypeCoercion(TypeCoercion),
    Variable(Variable),
}

impl Expression {
    pub fn source_information(&self) -> &Arc<SourceInformation> {
        match self {
            Self::Application(application) => application.source_information(),
            Self::Boolean(boolean) => boolean.source_information(),
            Self::Case(case) => case.source_information(),
            Self::RecordConstruction(record_construction) => {
                record_construction.source_information()
            }
            Self::RecordElementOperation(operation) => operation.source_information(),
            Self::RecordUpdate(record_update) => record_update.source_information(),
            Self::If(if_) => if_.source_information(),
            Self::Let(let_) => let_.source_information(),
            Self::LetRecursive(let_) => let_.source_information(),
            Self::List(list) => list.source_information(),
            Self::ListCase(case) => case.source_information(),
            Self::Operation(operation) => operation.source_information(),
            Self::String(string) => string.source_information(),
            Self::TypeCoercion(coercion) => coercion.source_information(),
            Self::None(none) => none.source_information(),
            Self::Number(number) => number.source_information(),
            Self::Variable(variable) => variable.source_information(),
        }
    }

    pub fn transform_expressions<E>(
        &self,
        transform: &mut impl FnMut(&Expression) -> Result<Expression, E>,
    ) -> Result<Self, E> {
        let expression = match self {
            Self::Application(application) => application.transform_expressions(transform)?.into(),
            Self::Case(case) => case.transform_expressions(transform)?.into(),
            Self::RecordConstruction(record_construction) => {
                record_construction.transform_expressions(transform)?.into()
            }
            Self::RecordElementOperation(operation) => {
                operation.transform_expressions(transform)?.into()
            }
            Self::RecordUpdate(record_update) => {
                record_update.transform_expressions(transform)?.into()
            }
            Self::If(if_) => if_.transform_expressions(transform)?.into(),
            Self::Let(let_) => let_.transform_expressions(transform)?.into(),
            Self::LetRecursive(let_) => let_.transform_expressions(transform)?.into(),
            Self::List(list) => list.transform_expressions(transform)?.into(),
            Self::ListCase(case) => case.transform_expressions(transform)?.into(),
            Self::Operation(operation) => operation.transform_expressions(transform)?.into(),
            Self::TypeCoercion(coercion) => coercion.transform_expressions(transform)?.into(),
            Self::Boolean(_)
            | Self::None(_)
            | Self::Number(_)
            | Self::String(_)
            | Self::Variable(_) => self.clone(),
        };

        transform(&expression)
    }

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Type) -> Result<Type, E>,
    ) -> Result<Self, E> {
        Ok(match self {
            Self::Application(application) => application.transform_types(transform)?.into(),
            Self::Case(case) => case.transform_types(transform)?.into(),
            Self::RecordConstruction(record_construction) => {
                record_construction.transform_types(transform)?.into()
            }
            Self::RecordElementOperation(operation) => operation.transform_types(transform)?.into(),
            Self::RecordUpdate(record_update) => record_update.transform_types(transform)?.into(),
            Self::If(if_) => if_.transform_types(transform)?.into(),
            Self::Let(let_) => let_.transform_types(transform)?.into(),
            Self::LetRecursive(let_) => let_.transform_types(transform)?.into(),
            Self::List(list) => list.transform_types(transform)?.into(),
            Self::ListCase(case) => case.transform_types(transform)?.into(),
            Self::Operation(operation) => operation.transform_types(transform)?.into(),
            Self::TypeCoercion(coercion) => coercion.transform_types(transform)?.into(),
            Self::Boolean(_)
            | Self::None(_)
            | Self::Number(_)
            | Self::String(_)
            | Self::Variable(_) => self.clone(),
        })
    }
}

impl From<Application> for Expression {
    fn from(application: Application) -> Self {
        Self::Application(application)
    }
}

impl From<Boolean> for Expression {
    fn from(boolean: Boolean) -> Self {
        Self::Boolean(boolean)
    }
}

impl From<Case> for Expression {
    fn from(case: Case) -> Self {
        Self::Case(case)
    }
}

impl From<EinString> for Expression {
    fn from(string: EinString) -> Self {
        Self::String(string)
    }
}

impl From<RecordConstruction> for Expression {
    fn from(record_construction: RecordConstruction) -> Self {
        Self::RecordConstruction(record_construction)
    }
}

impl From<RecordElementOperation> for Expression {
    fn from(operation: RecordElementOperation) -> Self {
        Self::RecordElementOperation(operation)
    }
}

impl From<RecordUpdate> for Expression {
    fn from(record_update: RecordUpdate) -> Self {
        Self::RecordUpdate(record_update)
    }
}

impl From<If> for Expression {
    fn from(if_: If) -> Self {
        Self::If(if_)
    }
}

impl From<Let> for Expression {
    fn from(let_: Let) -> Self {
        Self::Let(let_)
    }
}

impl From<LetRecursive> for Expression {
    fn from(let_: LetRecursive) -> Self {
        Self::LetRecursive(let_)
    }
}

impl From<List> for Expression {
    fn from(list: List) -> Self {
        Self::List(list)
    }
}

impl From<ListCase> for Expression {
    fn from(case: ListCase) -> Self {
        Self::ListCase(case)
    }
}

impl From<None> for Expression {
    fn from(none: None) -> Self {
        Self::None(none)
    }
}

impl From<Number> for Expression {
    fn from(number: Number) -> Self {
        Self::Number(number)
    }
}

impl<T: Into<Operation>> From<T> for Expression {
    fn from(operation: T) -> Self {
        Self::Operation(operation.into())
    }
}

impl From<TypeCoercion> for Expression {
    fn from(coercion: TypeCoercion) -> Self {
        Self::TypeCoercion(coercion)
    }
}

impl From<Variable> for Expression {
    fn from(variable: Variable) -> Self {
        Self::Variable(variable)
    }
}
