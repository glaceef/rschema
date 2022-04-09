use crate::Case;

pub trait Attribute {
    fn additional_properties(&self) -> bool;
    fn rename_all(&self) -> Option<Case>;
}