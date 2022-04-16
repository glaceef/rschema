#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
enum Enum {
    EmptyTupleVariant(),

    TupleVariant(
        #[rschema(
            minimum = 0,
            maximum = 100,
        )]
        i32,

        String,
    ),

    StructVariant {
        #[rschema(title = "i32")]
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
#[rschema(rename_all = "camelCase")]
enum EnumUnitsRenamed {
    UnitVariantX,
    UnitVariantY,
    UnitVariantZ,
}

#[derive(Debug, Schematic)]
struct EnumProperties {
    #[rschema(title = "Enum")]
    prop_enum: Enum,

    #[rschema(title = "EnumUnits")]
    prop_enum_units: EnumUnits,

    #[rschema(title = "EnumUnitsRenamed")]
    prop_enum_units_renamed: EnumUnitsRenamed,
}

#[test]
fn it_generates_enum_schema() -> rschema::Result<()> {
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
          "type": "array",
          "items": [
            {
              "type": "number",
              "minimum": 0,
              "maximum": 100
            },
            {
              "type": "string"
            }
          ],
          "minItems": 2,
          "maxItems": 2
        },
        {
          "type": "object",
          "properties": {
            "value": {
              "title": "i32",
              "type": "number"
            }
          },
          "additionalProperties": false
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
    },
    "prop_enum_units_renamed": {
      "title": "EnumUnitsRenamed",
      "type": "string",
      "enum": [
        "unitVariantX",
        "unitVariantY",
        "unitVariantZ"
      ]
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}