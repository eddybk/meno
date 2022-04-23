pub(crate) mod values;
pub(crate) mod statements;
pub(crate) mod runtime;

fn main() {
    use values::values::{Value, Side};


    let _value = Value::new(String::from("34"), Side::R);
    
}
