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

#[derive(Debug, Deserialize)]
pub enum PropType {
    String(StringProp),
    // Integer(NumericProp),
    Number(NumericProp),
    Boolean,
    Null,
    Array(ArrayProp),
    Object(ObjectProp),

    // 順序の定まっていない複合型
    Enum(EnumProp),

    // タプル構造体（順序の決まった複合型）
    Tuple(TupleProp),
}

macro_rules! prop_match_block {
    ($variant:ident, $inner:expr, $serializer:expr) => {
        {
            let wrapper = Wrapper {
                ty: stringify!($variant),
                inner: $inner,
            };
            wrapper.serialize($serializer)
        }
    };
}

macro_rules! unit_match_block {
    ($variant:ident, $serializer:expr) => {
        {
            let wrapper = Wrapper {
                ty: stringify!($variant),
                inner: &(),
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
            PropType::String( ref prop) => prop_match_block!( string, prop, serializer),
            // PropType::Integer(ref prop) => match_block!(integer, prop, serializer),
            PropType::Number( ref prop) => prop_match_block!( number, prop, serializer),
            PropType::Boolean           => unit_match_block!(boolean, serializer),
            PropType::Null              => unit_match_block!(   null, serializer),
            PropType::Array(  ref prop) => prop_match_block!(  array, prop, serializer),
            PropType::Object( ref prop) => prop_match_block!( object, prop, serializer),
            PropType::Enum(   ref prop) => prop.serialize(serializer),
            PropType::Tuple(  ref prop) => prop_match_block!(  tuple, prop, serializer),
        }
    }
}
