use darling::{
    FromAttributes,
    FromDeriveInput,
};

use crate::{
    Case,
    EnumAttribute,
    StructAttribute,
    is_falsy,
};

mod enum_attr;
mod struct_attr;

pub use enum_attr::EnumAttr;
pub use struct_attr::StructAttr;

#[derive(Debug, Default, FromAttributes, FromDeriveInput)]
#[darling(attributes(rschema))]
pub struct ContainerAttr {
    #[darling(default)]
    pub additional_properties: Option<bool>,

    #[darling(default)]
    pub rename_all: Option<Case>,
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
