#![allow(dead_code)]

pub mod statements {
    use crate::values::values::{Value as Variable, Number};

    pub enum Value {
        N(Number),
        S(String)
    }

    impl Value {
        pub fn from_variable(variable: Variable) -> Self {
            match variable {
                Variable::N(n) => Value::N(match n {
                    crate::values::values::ValueN::L(value) => value,
                    crate::values::values::ValueN::R(value) => value,
                }),
                Variable::S(s) => Value::S(match s {
                    crate::values::values::ValueS::L(value) => value,
                    crate::values::values::ValueS::R(value) => value,
                }),
            }
        }
    }

    pub struct Expression {
        values: Box<Vec<Value>>,
        aliases: Box<Vec<String>>
    }

    impl Expression {
        pub fn new() -> Self {

            todo!();
        }
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

    pub enum Statement {
        Declaration(Declaration),
        Expression(Expression)
    }

}