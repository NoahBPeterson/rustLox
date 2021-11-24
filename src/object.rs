use crate::value::Value;


#[derive(Clone)]
pub enum ObjType
{
    ObjString(Box<ObjString>),
}

#[derive(Clone)]
pub struct Obj
{
    pub typeOfObject: ObjType,
}

#[derive(Clone)]
pub struct ObjString
{
    pub length: u32,
    pub str: String,
}

impl Obj
{
    pub fn isObjType(value: Value, _objectType: ObjType) -> bool
    {
        if value.clone().IsObject()
        {
            match value.GetObject().typeOfObject
            {
                _objectType => return true,
                _ => return false,
            }
        }
        return false;
        // return IS_OBJ(value) && AS_OBJ(value)->typeOfObject == objectType;
    }

    pub fn CopyString(str: String, length: u32) -> Obj
    {
        Obj { typeOfObject: ObjType::ObjString(Box::from(ObjString { str: str, length: length }) )}
        // ObjString { str: str, length: length} } //, obj: Obj { typeOfObject: ObjType::ObjString } }

    }
}