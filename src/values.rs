#![allow(dead_code)]

pub mod Values {
    pub type Decimal = f64;

    pub enum ValueD {
        L(Decimal),
        R(Decimal)
    }
    pub enum ValueS {
        L(String),
        R(String)
    }
    pub enum Value {
        D(ValueD),
        S(ValueS)
    }

    pub struct Variables {
        values: Vec<Value>
    }

    impl Variables {
        pub fn new(values: Vec<Value>) -> Variables {
            Variables { values }
        }
        pub fn add(&mut self, value: Value) {
            self.values.push(value);
        }
    }

}