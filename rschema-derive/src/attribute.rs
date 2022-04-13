use crate::Case;

pub trait ContainerAttribute {
    fn definitions(&self) -> bool;
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
