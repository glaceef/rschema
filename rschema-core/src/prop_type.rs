use serde::{
    Serialize,
    Serializer,
    Deserialize,
};

mod array_prop;
mod enum_prop;
mod numeric_prop;
mod object_prop;
mod string_prop;
mod tuple_prop;

pub use array_prop::{
    ArrayProp,
    Items,
};
pub use enum_prop::EnumProp;
pub use numeric_prop::NumericProp;
pub use object_prop::ObjectProp;
pub use string_prop::StringProp;
pub use tuple_prop::TupleProp;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PropType {
    String(StringProp),
    Integer(NumericProp),
    Number(NumericProp),
    Boolean,
    Null,
    Array(ArrayProp),
    Object(ObjectProp),

    // 順序の定まっていない複合型
    // TODO: typeを出さないようにする
    Enum(EnumProp),

    // タプル構造体（順序の決まった複合型）
    Tuple(TupleProp),
}

/*
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PropType {
    String(string_prop::StringProp),
    Integer(numeric_prop::NumericProp),
    Number(numeric_prop::NumericProp),
    Boolean,
    Array(array_prop::ArrayProp),
    Object(object_prop::ObjectProp),
    Tuple(tuple_prop::TupleProp),
}

macro_rules! match_block {
    ($variant:ident, $target:expr, $serializer:expr) => {
        {
            let wrapper = Wrapper {
                ty: stringify!($variant),
                inner: $target,
            };
            wrapper.serialize($serializer)
        }
    };
}

#[derive(Serialize)]
struct Wrapper<'a, T> {
    #[serde(rename = "type")]
    ty: &'a str,

    #[serde(flatten)]
    inner: &'a T,
}

impl Serialize for PropType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PropType::String( ref prop) => match_block!( string, prop, serializer),
            PropType::Integer(ref prop) => match_block!(integer, prop, serializer),
            PropType::Number( ref prop) => match_block!( number, prop, serializer),
            PropType::Boolean           => serializer.serialize_str("boolean"),
            PropType::Array(  ref prop) => match_block!(  array, prop, serializer),
            PropType::Object( ref prop) => match_block!( object, prop, serializer),
            PropType::Tuple(  ref prop) => prop.serialize(serializer),
        }
    }
}
*/
