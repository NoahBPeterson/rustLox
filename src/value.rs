
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
    if IsNil(value)
    {
        return false;
    }
    value.Value != 0
}

pub fn GetNumber(value: Value) -> f64
{
    f64::from_be_bytes(u64::to_be_bytes(value.Value))
}

pub fn ValuesEqual(a: Value, b: Value) -> bool
{
    if a.ValueType as u8 != b.ValueType as u8
    {
        return false;
    }
    match a.ValueType as u8
    {
        x if x == ValueType::ValBool as u8 => return GetBool(a) == GetBool(b),
        x if x == ValueType::ValNil as u8 => return true,
        x if x == ValueType::ValNumber as u8 => return GetNumber(a) == GetNumber(b),
        _ => return false,
    }
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
    match value.ValueType as u8
    {
        x if x == ValueType::ValBool as u8 => 
        {
            if GetBool(value)
            {
                print!("true");
            }
            else
            {
                print!("false");
            }
        }
        x if x == ValueType::ValNil as u8 => print!("nil"),
        x if x == ValueType::ValNumber as u8 => print!("{}", GetNumber(value)),
        _ => print!("ValueType not matched! {}", value.ValueType as u8),
    }
}