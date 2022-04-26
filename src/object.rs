use crate::{value::{NilAsValue, Value}, vm::VM};


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

#[derive(Clone, Hash)]
pub struct ObjString
{
    pub length: u32,
    pub str: String,
}

impl PartialEq for ObjString {
    fn eq(&self, other: &Self) -> bool {
        self.str.eq(&other.str)
    }
}
impl Eq for ObjString {}

impl Obj
{
    pub fn isObjType(value: Value, _object_type: ObjType) -> bool
    {
        if value.clone().IsObject()
        {
            match value.GetObject().typeOfObject
            {
                _object_type => return true,
                _ => return false,
            }
        }
        return false;
        // return IS_OBJ(value) && AS_OBJ(value)->typeOfObject == objectType;
    }

    pub fn CopyString(vm: &mut VM, str: String, length: u32) -> Obj
    {
        vm.TableSet(ObjString { str: str.clone(), length: length }, NilAsValue() );
        Obj { typeOfObject: ObjType::ObjString(Box::from(ObjString { str: str, length: length }) )}
        // ObjString { str: str, length: length} } //, obj: Obj { typeOfObject: ObjType::ObjString } }
    }
}