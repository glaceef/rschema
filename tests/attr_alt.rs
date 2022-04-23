#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};
use uuid::Uuid;

#[derive(Debug, Schematic)]
struct AltAttribute {
    #[rschema(
        title = "UUID",
        alt = "String",
        pattern = r"[a-z0-9]{8}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{12}",
    )]
    uuid: Uuid,
}

#[test]
fn it_tests_alt_attribute() -> rschema::Result<()> {
    let schema_str = Schema::new::<AltAttribute>("Alt Attribute")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "Alt Attribute",
  "type": "object",
  "properties": {
    "uuid": {
      "title": "UUID",
      "type": "string",
      "pattern": "[a-z0-9]{8}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{12}"
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}