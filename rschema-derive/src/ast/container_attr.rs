use darling::{
    FromAttributes,
    FromDeriveInput,
};

use crate::{
    Case,
    ContainerAttribute,
    EnumAttribute,
    StructAttribute,
    TupleStructAttribute,
    is_falsy,
};

mod enum_attr;
mod newtype_struct_attr;
mod struct_attr;
mod tuple_struct_attr;
mod unit_struct_attr;

pub use enum_attr::EnumAttr;
pub use newtype_struct_attr::NewTypeStructAttr;
pub use struct_attr::StructAttr;
pub use tuple_struct_attr::TupleStructAttr;
pub use unit_struct_attr::UnitStructAttr;

#[derive(Debug, Default, FromAttributes, FromDeriveInput)]
#[darling(attributes(rschema))]
pub struct ContainerAttr {
    #[darling(default)]
    pub additional_properties: Option<bool>,

    #[darling(default)]
    pub rename_all: Option<Case>,

    #[darling(default)]
    pub unique_items: Option<bool>,

    // bool    : 通常の使い方。std::any::type_name によって決められた名前で $defs に登録するが、
    //           その名前がユニーク性を保証していないことを明記する。可能であれば name を指定する
    //           こと、exportする構造体では名前の衝突を防ぐため name を指定しないことを記載。
    // name =  : $defs のキー名を指定。衝突した場合の挙動を明記。
    // ref =   : 下記参照。$idと組み合わせることを記載。
    // https://json-schema.org/understanding-json-schema/structuring.html#ref
    #[darling(default)]
    pub definitions: Option<bool>,
}

impl From<EnumAttr> for ContainerAttr {
    fn from(attr: EnumAttr) -> Self {
        ContainerAttr {
            rename_all: attr.rename_all,
            definitions: attr.definitions,
            ..Default::default()
        }
    }
}

impl From<NewTypeStructAttr> for ContainerAttr {
    fn from(attr: NewTypeStructAttr) -> Self {
        ContainerAttr {
            definitions: attr.definitions,
            ..Default::default()
        }
    }
}

impl From<StructAttr> for ContainerAttr {
    fn from(attr: StructAttr) -> Self {
        ContainerAttr {
            additional_properties: attr.additional_properties,
            rename_all: attr.rename_all,
            definitions: attr.definitions,
            ..Default::default()
        }
    }
}

impl From<TupleStructAttr> for ContainerAttr {
    fn from(attr: TupleStructAttr) -> Self {
        ContainerAttr {
            unique_items: attr.unique_items,
            definitions: attr.definitions,
            ..Default::default()
        }
    }
}

impl From<UnitStructAttr> for ContainerAttr {
    fn from(_attr: UnitStructAttr) -> Self {
        ContainerAttr::default()
    }
}

impl ContainerAttribute for ContainerAttr {
    fn definitions(&self) -> bool {
        !is_falsy(&self.definitions)
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
