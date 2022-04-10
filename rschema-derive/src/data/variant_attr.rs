use darling::FromAttributes;

use crate::{
    Attribute,
    Case,
    is_falsy,
};

mod struct_variant_attr;
mod unit_variant_attr;

pub use struct_variant_attr::StructVariantAttr;
pub use unit_variant_attr::UnitVariantAttr;

#[derive(Debug, Default, FromAttributes)]
#[darling(attributes(rschema))]
pub struct VariantAttr {
    #[darling(default)]
    pub additional_properties: Option<bool>,

    #[darling(default)]
    pub rename: Option<String>,

    #[darling(default)]
    pub rename_all: Option<Case>,

    #[darling(default)]
    pub skip: Option<bool>,
}

impl From<StructVariantAttr> for VariantAttr {
    fn from(attr: StructVariantAttr) -> Self {
        VariantAttr {
            additional_properties: attr.additional_properties,
            rename_all: attr.rename_all,
            skip: attr.skip,
            ..Default::default()
        }
    }
}

impl From<UnitVariantAttr> for VariantAttr {
    fn from(attr: UnitVariantAttr) -> Self {
        VariantAttr {
            rename: attr.rename,
            skip: attr.skip,
            ..Default::default()
        }
    }
}

impl Attribute for VariantAttr {
    fn additional_properties(&self) -> bool {
        !is_falsy(&self.additional_properties)
    }

    fn rename_all(&self) -> Option<Case> {
        self.rename_all
    }
}
