use darling::{
    FromAttributes,
    FromDeriveInput,
};

use crate::{
    Case,
    EnumAttribute,
    StructAttribute,
    TupleStructAttribute,
    is_falsy,
};

mod empty_struct_attr;
mod enum_attr;
mod struct_attr;
mod tuple_struct_attr;

pub use empty_struct_attr::EmptyStructAttr;
pub use enum_attr::EnumAttr;
pub use struct_attr::StructAttr;
pub use tuple_struct_attr::TupleStructAttr;

#[derive(Debug, Default, FromAttributes, FromDeriveInput)]
#[darling(attributes(rschema))]
pub struct ContainerAttr {
    #[darling(default)]
    pub additional_properties: Option<bool>,

    #[darling(default)]
    pub rename_all: Option<Case>,

    #[darling(default)]
    pub unique_items: Option<bool>,
}

impl From<EmptyStructAttr> for ContainerAttr {
    fn from(_attr: EmptyStructAttr) -> Self {
        ContainerAttr::default()
    }
}

impl From<EnumAttr> for ContainerAttr {
    fn from(attr: EnumAttr) -> Self {
        ContainerAttr {
            rename_all: attr.rename_all,
            ..Default::default()
        }
    }
}

impl From<StructAttr> for ContainerAttr {
    fn from(attr: StructAttr) -> Self {
        ContainerAttr {
            additional_properties: attr.additional_properties,
            rename_all: attr.rename_all,
            ..Default::default()
        }
    }
}

impl From<TupleStructAttr> for ContainerAttr {
    fn from(attr: TupleStructAttr) -> Self {
        ContainerAttr {
            unique_items: attr.unique_items,
            ..Default::default()
        }
    }
}

impl EnumAttribute for ContainerAttr {
    fn rename_all(&self) -> Option<Case> {
        self.rename_all
    }
}

impl StructAttribute for ContainerAttr {
    fn additional_properties(&self) -> bool {
        !is_falsy(&self.additional_properties)
    }

    fn rename_all(&self) -> Option<Case> {
        self.rename_all
    }
}

impl TupleStructAttribute for ContainerAttr {
    fn unique_items(&self) -> Option<bool> {
        self.unique_items
    }
}
