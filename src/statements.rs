pub mod Statements {
    use crate::values::Values::{Value as Variable, Decimal};

    pub enum Value {
        N(Decimal),
        S(String)
    }
    pub struct Expression {
        values: Vec<Value>,
        aliases: [String]
    }

    pub struct Declaration {
        values: (Variable, Variable)
    }
}