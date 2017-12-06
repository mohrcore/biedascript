use super::scope;
use super::procedure;

use ::std::str::FromStr;

pub struct Program {
    pub stack: Vec<scope::ScopedValue>,
    pub procedures: Vec<procedure::Procedure>,
}

impl Program {

    pub fn new() -> Self {
        Program {
            stack: Vec::new(),
            procedures: Vec::new(),
        }
    }

   pub fn execute<E>(&mut self, debug: bool) -> Result<(), E>
    where E: FromStr {

        if self.procedures.is_empty() {
            return Err(E::from_str("No procedures found in program!").unwrap_or_else(|_| panic!("Couldn't create an error!")))
        }
        self.stack = self.procedures[0].execute(Vec::new(), &self.procedures, debug)?;
        Ok(())
    }
}