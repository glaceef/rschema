#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
struct NewTypeStruct(
    #[rschema(skip)]
    i32,
);

#[derive(Debug, Schematic)]
struct TupleStruct(
    #[rschema(skip)]
    i32,
    String,
);

#[derive(Debug, Schematic)]
enum Enum {
    Variant1(i32, String),
    #[rschema(skip)]
    Variant2 {
        value: i32,
    }
}

#[derive(Debug, Schematic)]
enum UnitVariantsEnum {
    UnitVariant1,
    #[rschema(skip)]
    UnitVariant2,
    UnitVariant3,
}

#[derive(Debug, Schematic)]
#[rschema(additional_properties)]
struct Attributes {
    #[rschema(
        minimum = 0,
        maximum = 100,
    )]
    prop_no_title: i32,

    prop_no_attr: i32,

    #[rschema(skip)]
    prop_skip: i32,

    prop_skip_new_type_struct: NewTypeStruct,

    prop_skip_tuple_struct: TupleStruct,

    prop_skip_enum: Enum,

    prop_skip_unit_variants: UnitVariantsEnum,
}

#[test]
fn it_tests_attributes() -> rschema::Result<()> {
    let schema_str = Schema::new::<Attributes>("Attributes")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "Attributes",
  "type": "object",
  "properties": {
    "prop_no_title": {
      "type": "number",
      "minimum": 0,
      "maximum": 100
    },
    "prop_no_attr": {
      "type": "number"
    },
    "prop_skip_new_type_struct": {
      "type": "array",
      "items": [],
      "minItems": 0,
      "maxItems": 0
    },
    "prop_skip_tuple_struct": {
      "type": "array",
      "items": [
        {
          "type": "string"
        }
      ],
      "minItems": 1,
      "maxItems": 1
    },
    "prop_skip_enum": {
      "anyOf": [
        {
          "type": "array",
          "items": [
            {
              "type": "number"
            },
            {
              "type": "string"
            }
          ],
          "minItems": 2,
          "maxItems": 2
        }
      ]
    },
    "prop_skip_unit_variants": {
      "type": "string",
      "enum": [
        "UnitVariant1",
        "UnitVariant3"
      ]
    }
  },
  "additionalProperties": true
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}