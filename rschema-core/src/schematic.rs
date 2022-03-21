use crate::types::Properties;

type Required = &'static[&'static str];

pub trait Schematic {
    const PROPERTIES_STR: &'static str;
    const REQUIRED: Required;
    const ADDITIONAL_PROPERTIES: bool;

    // serde_json を隠ぺいするため
    fn restore_properties() -> Properties {
        serde_json::from_str(Self::PROPERTIES_STR).unwrap()
    }

    fn properties() -> Properties;

    fn required() -> Vec<String> {
        Self::REQUIRED
            .into_iter()
            .map(|&s| s.into() )
            .collect()
    }

    fn additional_properties() -> bool {
        Self::ADDITIONAL_PROPERTIES
    }
}