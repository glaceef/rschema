use crate::{
    Case,
    Definitions,
};

pub trait ContainerAttribute {
    fn definitions(&self) -> &Definitions;
}

pub trait EnumAttribute {
    fn rename_all(&self) -> Option<Case>;
}

pub trait StructAttribute {
    fn additional_properties(&self) -> bool;
    fn rename_all(&self) -> Option<Case>;
}

pub trait TupleStructAttribute {
    fn unique_items(&self) -> Option<bool>;
}
