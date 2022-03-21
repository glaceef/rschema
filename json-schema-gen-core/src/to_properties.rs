use crate::types::Properties;

pub trait ToProperties {
    const PROPERTIES_STR: &'static str;

    // serde_json を隠ぺいするため
    fn restore_properties() -> Properties {
        serde_json::from_str(Self::PROPERTIES_STR).unwrap()
    }

    fn to_properties() -> Properties;
}