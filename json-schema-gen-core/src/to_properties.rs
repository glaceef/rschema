use crate::types::Properties;

type Required = &'static[&'static str];

pub trait ToProperties {
    const PROPERTIES_STR: &'static str;
    const REQUIRED: Required;

    // serde_json を隠ぺいするため
    fn restore_properties() -> Properties {
        serde_json::from_str(Self::PROPERTIES_STR).unwrap()
    }

    fn to_properties() -> Properties;
}