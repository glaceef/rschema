mod struct_variant_attr;
mod unit_variant_attr;

pub use struct_variant_attr::StructVariantAttr;
pub use unit_variant_attr::UnitVariantAttr;

#[derive(Debug, Default)]
pub struct VariantAttr {
    pub additional_properties: bool,
}

impl From<StructVariantAttr> for VariantAttr {
    fn from(attr: StructVariantAttr) -> Self {
        VariantAttr {
            additional_properties: attr.additional_properties,
        }
    }
}

impl From<UnitVariantAttr> for VariantAttr {
    fn from(_attr: UnitVariantAttr) -> Self {
        VariantAttr {
            ..Default::default()
        }
    }
}