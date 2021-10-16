
#[derive(Clone)]
pub struct ValueArray
{
    pub values: Vec<Value>,
}

#[derive(Clone, Copy)]
pub enum ValueType
{
    ValBool = 1,
    ValNil = 2,
    ValNumber = 3,
}

#[derive(Clone, Copy)]
pub struct Value
{
    ValueType: ValueType,
    Value: u64,
}

pub fn BoolAsValue(boolean: bool) -> Value
{
    Value { ValueType: ValueType::ValBool, Value: u64::from_be_bytes(u64::to_be_bytes(boolean as u64))}
}

pub fn NilAsValue() -> Value
{
    Value { ValueType: ValueType::ValNil, Value: u64::from_be_bytes(u64::to_be_bytes(0 as u64))}
}

pub fn NumberAsValue(number: f64) -> Value
{
    Value { ValueType: ValueType::ValNumber, Value: u64::from_be_bytes(f64::to_be_bytes(number))}
}

pub fn IsBool(value: Value) -> bool
{
    if value.ValueType as u8 == ValueType::ValBool as u8
    {
        return true;
    }
    return false;
}

pub fn IsNumber(value: Value) -> bool
{
    if value.ValueType as u8 == ValueType::ValNumber as u8
    {
        return true;
    }
    return false;
}

pub fn IsNil(value: Value) -> bool
{
    if value.ValueType as u8 == ValueType::ValNil as u8
    {
        return true;
    }
    return false;
}

pub fn GetBool(value: Value) -> bool
{
    value.Value != 0
}

pub fn GetNumber(value: Value) -> f64
{
    f64::from_be_bytes(u64::to_be_bytes(value.Value))
}

pub fn init_value_array() -> ValueArray
{
    return ValueArray {values: Vec::with_capacity(0) };
}

pub fn write_value_array(value_array: &mut ValueArray, value: f64)
{
    value_array.values.push(NumberAsValue(value));
}

pub fn print_value(value: Value)
{
    print!("{}", GetNumber(value));
}