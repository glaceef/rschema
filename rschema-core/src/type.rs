use serde::{
    Serialize,
    Serializer,
};

mod array_keys;
mod enum_keys;
mod numeric_keys;
mod object_keys;
mod string_keys;
mod tuple_keys;

pub use array_keys::{
    ArrayKeys,
    Items,
};
pub use enum_keys::EnumKeys;
pub use numeric_keys::NumericKeys;
pub use object_keys::{
    AdditionalProperties,
    ObjectKeys,
    Properties,
    Property,
};
pub use string_keys::StringKeys;
pub use tuple_keys::TupleKeys;

/// Represents some property type.
/// 
#[derive(Debug)]
pub enum Type {
    /// For a `string` type property.
    /// 
    String(StringKeys),

    /// For a `number` type property.
    /// 
    // Integer(NumericKeys),
    Number(NumericKeys),

    /// For a `boolean` type property.
    /// 
    Boolean,

    /// For a `null` type property.
    /// 
    Null,

    /// For an `array` type property.
    /// 
    Array(ArrayKeys),

    /// For an `object` type property.
    /// 
    Object(ObjectKeys),

    /// For an `array` type property. In particular, it has unordered and composite type items.
    /// 
    Enum(EnumKeys),

    /// For an `array` type property. In particular, it has ordered and composite type items.
    /// 
    Tuple(TupleKeys),
}

macro_rules! keys_match_block {
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

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::String(ref keys) => keys_match_block!( string, keys, serializer),
            Self::Number(ref keys) => keys_match_block!( number, keys, serializer),
            Self::Boolean          => unit_match_block!(boolean, serializer),
            Self::Null             => unit_match_block!(   null, serializer),
            Self::Array( ref keys) => keys_match_block!(  array, keys, serializer),
            Self::Object(ref keys) => keys_match_block!( object, keys, serializer),
            Self::Enum(  ref keys) => keys.serialize(serializer),
            Self::Tuple( ref keys) => keys_match_block!(  tuple, keys, serializer),
        }
    }
}
