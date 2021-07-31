
#[derive(Clone)]
pub struct ValueArray
{
    pub values: Vec<f64>,
}

pub fn init_value_array() -> ValueArray
{
    return ValueArray {values: Vec::with_capacity(0) };
}

pub fn write_value_array(value_array: &mut ValueArray, value: f64)
{
    value_array.values.push(value);
}

pub fn print_value(value: f64)
{
    print!("{}", value);
}