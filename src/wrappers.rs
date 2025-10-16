use serde::ser::SerializeMap;
use windows_metadata::{reader::{MethodDef, MethodParam}, HasAttributes, ParamAttributes, Type};

use crate::constants::get_type_variant_index;

pub struct TypeWrapper(pub Type);

impl serde::ser::Serialize for TypeWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry("type", &get_type_variant_index(& self.0))?;
        match & self.0 {
            Type::Name(name) => {
                s.serialize_entry("name", &name.name)?;
            }
            Type::PtrMut(ty, _) => {
                s.serialize_entry("inner", &TypeWrapper((**ty).clone()))?;
            }
            Type::PtrConst(ty, _) => {
                s.serialize_entry("inner", &TypeWrapper((**ty).clone()))?;
            }
            Type::ConstRef(ty) => {
                s.serialize_entry("inner", &TypeWrapper((**ty).clone()))?;
            }
            _ => (),
        }

        s.end()
    }
}

pub struct ParamAttributesWrapper(pub ParamAttributes);
impl serde::ser::Serialize for ParamAttributesWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut s = serializer.serialize_map(Some(3))?;
        s.serialize_entry("in", &self.0.contains(ParamAttributes::In))?;
        s.serialize_entry("out", &self.0.contains(ParamAttributes::Out))?;
        s.serialize_entry("optional", &self.0.contains(ParamAttributes::Optional))?;
        s.end()
    }
}

pub struct MethodParamWrapper<'a> {
    pub param: MethodParam<'a>,
    pub ty: Type,
}
impl<'a> serde::ser::Serialize for MethodParamWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut s = serializer.serialize_map(Some(4))?;
        s.serialize_entry("name", &self.param.name())?;
        s.serialize_entry("type", &TypeWrapper(self.ty.clone()))?;
        s.serialize_entry("attributes", &ParamAttributesWrapper(self.param.flags()))?;
        s.serialize_entry("constant", &self.param.has_attribute("ConstAttribute"))?;
        s.end()
    }
}

pub struct MethodDefWrapper<'a>(pub MethodDef<'a>);
impl<'a> serde::ser::Serialize for MethodDefWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let signature = self.0.signature(&[]);
        let mut s = serializer.serialize_map(Some(5))?;
        s.serialize_entry("name", &self.0.name())?;
        s.serialize_entry("return_type", &TypeWrapper(signature.return_type.clone()))?;
        let params = std::iter::zip(self.0.params().skip(1), signature.types).map(|(param, ty)| MethodParamWrapper { param, ty: ty.clone() }).collect::<Vec<_>>();
        s.serialize_entry("params", &params)?;
        s.serialize_entry("is_static", &self.0.has_attribute("Static"))?;
        s.serialize_entry("is_virtual", &self.0.has_attribute("Virtual"))?;
        s.end()
    }
}