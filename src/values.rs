#![allow(dead_code)]

pub mod values {
    pub type Number = f64;

    pub enum ValueN {
        L(Number),
        R(Number)
    }
    pub enum ValueS {
        L(String),
        R(String)
    }
    pub enum Value {
        N(ValueN),
        S(ValueS)
    }
    pub enum Side {
        L,
        R
    }

    impl Value {
        pub fn new(value: String, side: Side) -> Value{
            match value.parse::<f64>() {
                Ok(v) => {
                    match side {
                        Side::L => {
                            return Value::N(ValueN::L(v));
                        }
                        Side::R => {
                            return Value::N(ValueN::R(v));
                        }
                    }
                }
                Err(_) => {
                    match side {
                        Side::L => {
                            return Value::S(ValueS::L(value));
                        }
                        Side::R => {
                            return Value::S(ValueS::R(value));
                        }
                    }
                },
            }
        }
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