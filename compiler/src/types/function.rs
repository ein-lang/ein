use super::Type;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    argument: Rc<Type>,
    result: Rc<Type>,
}

impl Function {
    pub fn new(argument: Type, result: Type) -> Self {
        Self {
            argument: Rc::new(argument),
            result: Rc::new(result),
        }
    }

    pub fn argument(&self) -> &Type {
        &self.argument
    }

    pub fn result(&self) -> &Type {
        &self.result
    }

    pub fn arguments(&self) -> Vec<&Type> {
        let mut arguments: Vec<&Type> = vec![&self.argument];
        let mut result: &Type = &self.result;

        while let Type::Function(function) = result {
            arguments.push(&function.argument);
            result = &function.result;
        }

        arguments
    }

    pub fn last_result(&self) -> &Type {
        match &*self.result {
            Type::Function(function) => function.result(),
            _ => &self.result,
        }
    }
}
