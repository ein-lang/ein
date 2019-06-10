use super::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    argument: Box<Type>,
    result: Box<Type>,
}

impl Function {
    pub fn new(argument: Type, result: Type) -> Self {
        Self {
            argument: Box::new(argument),
            result: Box::new(result),
        }
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

    pub fn result(&self) -> &Type {
        match self.result.as_ref() {
            Type::Function(function) => function.result(),
            _ => &self.result,
        }
    }
}
