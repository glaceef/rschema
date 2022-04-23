#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
#[rschema(rename_all = "camelCase")]
struct Struct {
    prop_value1: i32,
    prop_value2: String,

    #[rschema(rename = "prop-value-renamed")]
    prop_value3: bool,
}

#[derive(Debug, Schematic)]
#[rschema(rename_all = "Train-Case")]
enum Enum {
    UnitVariant1,
    UnitVariant2,

    #[rschema(rename = "unit_variant_3_renamed")]
    UnitVariant3,
}

#[derive(Debug, Schematic)]
struct RenameAttribute {
    #[rschema(rename = "prop_value_renamed")]
    prop_value: i32,

    prop_struct: Struct,

    prop_enum: Enum,
}

#[test]
fn it_tests_rename_attribute() -> rschema::Result<()> {
    let schema_str = Schema::new::<RenameAttribute>("Rename Attribute")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "Rename Attribute",
  "type": "object",
  "properties": {
    "prop_value_renamed": {
      "type": "number"
    },
    "prop_struct": {
      "type": "object",
      "properties": {
        "propValue1": {
          "type": "number"
        },
        "propValue2": {
          "type": "string"
        },
        "prop-value-renamed": {
          "type": "boolean"
        }
      },
      "additionalProperties": false
    },
    "prop_enum": {
      "type": "string",
      "enum": [
        "Unit-Variant-1",
        "Unit-Variant-2",
        "unit_variant_3_renamed"
      ]
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}