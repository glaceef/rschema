use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    Schematic,
    types::Properties,
};

mod other_props;
mod string_prop;
mod numeric_prop;
mod array_prop;
mod object_prop;

use other_props::OtherProps;

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    // 共通項目
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    // default: Option<>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // deprecated: Option<bool>,
    // r#const: Option<>,

    // その他の型ごとに異なる項目
    #[serde(flatten)]
    other_props: OtherProps,
}

impl Property {
    pub fn is_object(&self) -> bool {
        match self.other_props {
            OtherProps::Object(_) => true,
            _ => false,
        }
    }

    pub fn set_properties<T: Schematic>(&mut self) -> &mut Self {
        if let OtherProps::Object(ref mut prop) = self.other_props {
            prop.set_properties(T::properties());
        }
        self
    }

    pub fn set_required<T: Schematic>(&mut self) -> &mut Self {
        if let OtherProps::Object(ref mut prop) = self.other_props {
            prop.set_required(T::required());
        }
        self
    }

    pub fn set_additional_properties<T: Schematic>(&mut self) -> &mut Self {
        if let OtherProps::Object(ref mut prop) = self.other_props {
            prop.set_additional_properties(T::additional_properties());
        }
        self
    }
}
