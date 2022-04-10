use crate::{
    Attribute,
    Case,
    is_falsy,
};

mod other_variant_attr;
mod struct_variant_attr;

pub use other_variant_attr::OtherVariantAttr;
pub use struct_variant_attr::StructVariantAttr;

#[derive(Debug, Default)]
pub struct VariantAttr {
    pub additional_properties: Option<bool>,
    pub rename_all: Option<Case>,
    pub skip: Option<bool>,
}

impl From<StructVariantAttr> for VariantAttr {
    fn from(attr: StructVariantAttr) -> Self {
        VariantAttr {
            additional_properties: attr.additional_properties,
            rename_all: attr.rename_all,
            skip: attr.skip,
        }
    }
}

impl From<OtherVariantAttr> for VariantAttr {
    fn from(attr: OtherVariantAttr) -> Self {
        VariantAttr {
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
