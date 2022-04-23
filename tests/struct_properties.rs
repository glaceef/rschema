#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
struct UnitStruct;

#[derive(Debug, Schematic)]
struct EmptyTupleStruct();

#[derive(Debug, Schematic)]
struct NewTypeStruct(i32);

#[derive(Debug, Schematic)]
struct TupleStruct(i32, String);

#[derive(Debug, Schematic)]
struct NestedStruct {
    #[rschema(title = "i32")]
    value: i32,
}

#[derive(Debug, Schematic)]
struct StructProperties {
    #[rschema(title = "UnitStruct")]
    prop_unit_struct: UnitStruct,

    #[rschema(title = "EmptyTupleStruct")]
    prop_empty_tuple_struct: EmptyTupleStruct,

    #[rschema(title = "NewTypeStruct")]
    prop_new_type_struct: NewTypeStruct,

    #[rschema(title = "TupleStruct")]
    prop_tuple_struct: TupleStruct,

    #[rschema(title = "NestedStruct")]
    prop_nested_struct: NestedStruct,
}

#[test]
fn it_tests_struct_properties() -> rschema::Result<()> {
    let schema_str = Schema::new::<StructProperties>("Struct Properties")
        .to_string_pretty()?;
    let schema_str2 = r#"{
  "title": "Struct Properties",
  "type": "object",
  "properties": {
    "prop_unit_struct": {
      "title": "UnitStruct",
      "type": "null"
    },
    "prop_empty_tuple_struct": {
      "title": "EmptyTupleStruct",
      "type": "array",
      "items": [],
      "minItems": 0,
      "maxItems": 0
    },
    "prop_new_type_struct": {
      "title": "NewTypeStruct",
      "type": "number"
    },
    "prop_tuple_struct": {
      "title": "TupleStruct",
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
    },
    "prop_nested_struct": {
      "title": "NestedStruct",
      "type": "object",
      "properties": {
        "value": {
          "title": "i32",
          "type": "number"
        }
      },
      "additionalProperties": false
    }
  },
  "additionalProperties": false
}"#;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}