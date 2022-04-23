pub(crate) mod values;
pub(crate) mod statements;

fn main() {
    use values::Values::{Value, ValueS, ValueD};


    let value = Value::D(ValueD::R(3.));

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
