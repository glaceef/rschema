use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    PropType,
    Schematic,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub title: String,
    pub description: Option<String>,

    // 起点がenumの場合はtypeがない
    // シリアライズするとなくなる。
    #[serde(flatten)]
    pub ty: PropType,
}

impl Schema {
    pub fn new<T, S>(title: S) -> Self
    where
        T: Schematic,
        S: Into<String>,
    {
        Schema {
            title: title.into(),
            description: None,
            ty: T::ty2(),
        }
    }
}
