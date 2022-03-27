use serde::{
    Serialize,
    Deserialize,
};

use crate::prop_type::PropType;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Items {
    Single(PropType), // Vec<String> のような単一の配列
    Tuple(Vec<PropType>), // Tuple(u32, String) のようなタプル構造体
}