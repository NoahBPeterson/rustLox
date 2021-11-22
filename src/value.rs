use crate::object::Obj;


#[derive(Clone)]
pub struct ValueArray
{
    pub values: Vec<Value>,
}


#[derive(Clone)]
pub enum ValueType
{
    ValBool(bool),
    ValNil,
    ValNumber(f64),
    ValObj(Box<Obj>),
}

#[derive(Clone)]
pub struct Value
{
    ValueType: ValueType,
}

impl Value
{
    pub fn IsBool(self) -> bool
    {
        match self.ValueType
        {
            ValueType::ValBool(_) => return true,
            _ => return false,
        }
    }

    pub fn IsNumber(self) -> bool
    {
        match self.ValueType
        {
            ValueType::ValNumber(_) => return true,
            _ => return false,
        }
    }

    pub fn IsNil(self) -> bool
    {
        match self.ValueType
        {
            ValueType::ValNil => return true,
            _ => return false,
        }
    }

    pub fn IsObject(self) -> bool
    {
        match self.ValueType
        {
            ValueType::ValObj(_) => return true,
            _ => return false,
        }
    }

    pub fn GetBool(self) -> bool
    {
        match self.ValueType
        {
            ValueType::ValBool(val) => return val,
            _ => panic!("Attempted to get a bool from a non-bool!"),
        }
    }

    pub fn GetNumber(self) -> f64
    {
        match self.ValueType
        {
            ValueType::ValNumber(val) => return val,
            _ => panic!("Attempted to get a number from a non-number!"),
        }
    }

    pub fn GetObject(self) -> Obj
    {
        match self.ValueType
        {
            ValueType::ValObj(val) => return *val,
            _ => panic!("Attempted to get a number from a non-object!"),
        }
    }

    pub fn Equals(self, b: Value) -> bool
    {
        match (self.ValueType, b.ValueType)
        {
            (ValueType::ValBool(ValueOfA), ValueType::ValBool(ValueOfB)) => return ValueOfA == ValueOfB,
            (ValueType::ValNil, ValueType::ValNil) => return true,
            (ValueType::ValNumber(ValueOfA), ValueType::ValNumber(ValueOfB)) => return ValueOfA == ValueOfB,
            _ => return false,
        }
    }

    pub fn IsFalsey(self) -> bool
    {
        return self.clone().IsNil() || (self.clone().IsBool() && !self.clone().GetBool());
    }
}

impl From<bool> for Value
{
    fn from(boolean: bool) -> Self
    {
        Value { ValueType: ValueType::ValBool(boolean) }
    }
}

pub fn BoolAsValue(boolean: bool) -> Value
{
    Value { ValueType: ValueType::ValBool(boolean) }
}

pub fn NilAsValue() -> Value
{
    Value { ValueType: ValueType::ValNil}
}

pub fn NumberAsValue(number: f64) -> Value
{
    Value { ValueType: ValueType::ValNumber(number)}
}

pub fn ObjAsValue(obj: Obj) -> Value
{
    Value { ValueType: ValueType::ValObj(Box::from(obj)) }
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
    match value.ValueType
    {
        ValueType::ValBool(_) => 
        {
            if value.GetBool()
            {
                print!("true");
            }
            else
            {
                print!("false");
            }
        }
        ValueType::ValNil => print!("nil"),
        ValueType::ValNumber(_) => print!("{}", value.GetNumber()),
        _ => print!("ValueType not matched!"),
    }
}