#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
enum Enum {
    EmptyTupleVariant(),

    NewTypeVariant(i32),

    TupleVariant(
        #[rschema(
            title = "value",
            format = "myformat",
        )]
        String,

        bool,
    ),

    #[rschema(additional_properties)]
    StructVariant {
        #[rschema(title = "value")]
        value: i32,
    }
}

#[derive(Debug, Schematic)]
enum EnumUnits {
    UnitVariant1,
    UnitVariant2,
    UnitVariant3,
}

#[derive(Debug, Schematic)]
struct EnumProperties {
    #[rschema(title = "Enum")]
    prop_enum: Enum,

    #[rschema(title = "EnumUnits")]
    prop_enum_units: EnumUnits,
}

#[test]
fn it_tests_enum_properties() -> rschema::Result<()> {
    let schema_str = Schema::new::<EnumProperties>("Enum Properties")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "Enum Properties",
  "type": "object",
  "properties": {
    "prop_enum": {
      "title": "Enum",
      "anyOf": [
        {
          "type": "array",
          "items": [],
          "minItems": 0,
          "maxItems": 0
        },
        {
          "type": "number"
        },
        {
          "type": "array",
          "items": [
            {
              "title": "value",
              "type": "string",
              "format": "myformat"
            },
            {
              "type": "boolean"
            }
          ],
          "minItems": 2,
          "maxItems": 2
        },
        {
          "type": "object",
          "properties": {
            "value": {
              "title": "value",
              "type": "number"
            }
          },
          "additionalProperties": true
        }
      ]
    },
    "prop_enum_units": {
      "title": "EnumUnits",
      "type": "string",
      "enum": [
        "UnitVariant1",
        "UnitVariant2",
        "UnitVariant3"
      ]
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}