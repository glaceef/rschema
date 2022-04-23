#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
struct SkipAttribute {
    #[rschema(title = "value")]
    prop_value: i32,

    #[rschema(skip)]
    prop_ignored_value: i32,
}

#[test]
fn it_tests_skip_attribute() -> rschema::Result<()> {
    let schema_str = Schema::new::<SkipAttribute>("Skip Attribute")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "Skip Attribute",
  "type": "object",
  "properties": {
    "prop_value": {
      "title": "value",
      "type": "number"
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}