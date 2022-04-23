#![allow(dead_code)]

pub mod runtime {
    use crate::values::values::Value as Varaible;
    use crate::statements::statements::Statement;
    pub struct Runtime {
        scope_n: u32,
        scope_c: usize,
        variables: Vec<Vec<Varaible>>,
        statements: Vec<Vec<Statement>>
    }
}