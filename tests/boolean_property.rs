#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
struct BooleanProperty {
    #[rschema(title = "bool")]
    prop_bool: bool,
}

#[test]
fn it_generates_boolean_schema() -> rschema::Result<()> {
    let schema_str = Schema::new::<BooleanProperty>("Boolean Property")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "Boolean Property",
  "type": "object",
  "properties": {
    "prop_bool": {
      "title": "bool",
      "type": "boolean"
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}