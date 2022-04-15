use darling::FromAttributes;

use crate::{
    Case,
    ContainerAttribute,
    StructAttribute,
    TupleStructAttribute,
    is_falsy,
};

mod other_variant_attr;
mod struct_variant_attr;
mod tuple_struct_variant_attr;
mod unit_variant_attr;

pub use other_variant_attr::OtherVariantAttr;
pub use struct_variant_attr::StructVariantAttr;
pub use tuple_struct_variant_attr::TupleStructVariantAttr;
pub use unit_variant_attr::UnitVariantAttr;

#[derive(Debug, Default, PartialEq, FromAttributes)]
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

    #[darling(default)]
    pub unique_items: Option<bool>,
}

impl From<OtherVariantAttr> for VariantAttr {
    fn from(attr: OtherVariantAttr) -> Self {
        VariantAttr {
            skip: attr.skip,
            ..Default::default()
        }
    }
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

impl From<TupleStructVariantAttr> for VariantAttr {
    fn from(attr: TupleStructVariantAttr) -> Self {
        VariantAttr {
            skip: attr.skip,
            unique_items: attr.unique_items,
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

impl ContainerAttribute for VariantAttr {
    fn definitions(&self) -> bool {
        false
    }
}

impl StructAttribute for VariantAttr {
    fn additional_properties(&self) -> bool {
        !is_falsy(&self.additional_properties)
    }

    fn rename_all(&self) -> Option<Case> {
        self.rename_all
    }
}

impl TupleStructAttribute for VariantAttr {
    fn unique_items(&self) -> Option<bool> {
        self.unique_items
    }
}
