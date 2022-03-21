use serde::{
    Serialize,
    Deserialize,
};

use super::{
    string_prop::StringProp,
    numeric_prop::NumericProp,
    array_prop::ArrayProp,
    object_prop::ObjectProp,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum OtherProps {
    String(StringProp),
    Number(NumericProp),
    Integer(NumericProp),
    Array(ArrayProp),
    Object(ObjectProp),
    Boolean,
    Null, // ?
    // Unknown,
}