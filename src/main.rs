pub(crate) mod values;
pub(crate) mod statements;

fn main() {
    use values::Values::{Value, ValueS, ValueN, Side};


    let value = Value::new(String::from("34"), Side::R);
    
    /*match value {
        Value::D(decimal) => {
            match decimal {
                ValueD::L(value) => {

                }
                ValueD::R(value) => {

                }
            }
        }
        Value::S(string) => {
            match string {
                ValueS::L(value) => {
                    
                }
                ValueS::R(value) => {

                }
            }
        }
    }*/



}
