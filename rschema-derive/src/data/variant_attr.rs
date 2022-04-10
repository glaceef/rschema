use crate::{
    Attribute,
    Case,
    is_falsy,
};

mod struct_variant_attr;
mod empty_variant_attr;

pub use struct_variant_attr::StructVariantAttr;
pub use empty_variant_attr::EmptyVariantAttr;

#[derive(Debug, Default)]
pub struct VariantAttr {
    pub additional_properties: Option<bool>,
    pub rename_all: Option<Case>,
}

impl From<StructVariantAttr> for VariantAttr {
    fn from(attr: StructVariantAttr) -> Self {
        VariantAttr {
            additional_properties: attr.additional_properties,
            rename_all: attr.rename_all,
        }
    }
}

impl From<EmptyVariantAttr> for VariantAttr {
    fn from(_attr: EmptyVariantAttr) -> Self {
        Self::default()
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
