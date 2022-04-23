pub mod Statements {
    use crate::values::Values::{Value as Variable, Number};

    pub enum Value {
        N(Number),
        S(String)
    }
    pub struct Expression {
        values: Vec<Value>,
        aliases: [String]
    }

    pub enum Type {
        Function,
        String,
        Number
    }

    pub struct Declaration {
        values: (Variable, Variable),
        dec_type: Type
    }
}