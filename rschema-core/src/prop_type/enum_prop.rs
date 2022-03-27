use serde::{
    Serialize,
    Deserialize,
};

use super::PropType;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct EnumProp {
    pub any_of: Vec<PropType>,
}
