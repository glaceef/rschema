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
struct TupleStruct(
    #[rschema(
        minimum = 0,
        maximum = 100,
    )]
    i32,

    String,
);

#[derive(Debug, Schematic)]
struct NestedStruct {
    #[rschema(title = "i32")]
    value: i32,
}

#[derive(Debug, Schematic)]
#[rschema(rename_all = "Train-Case")]
struct RenamedStruct {
    #[rschema(title = "i32")]
    value_number: i32,

    #[rschema(title = "String")]
    value_string: String,
}

#[derive(Debug, Schematic)]
struct StructProperties {
    #[rschema(title = "UnitStruct")]
    prop_unit_struct: UnitStruct,

    #[rschema(title = "EmptyTupleStruct")]
    prop_empty_tuple_struct: EmptyTupleStruct,

    #[rschema(title = "TupleStruct")]
    prop_tuple_struct: TupleStruct,

    #[rschema(title = "NestedStruct")]
    prop_nested_struct: NestedStruct,

    #[rschema(title = "RenamedStruct")]
    prop_renamed_struct: RenamedStruct,
}

#[test]
fn it_generates_struct_schema() -> rschema::Result<()> {
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
    "prop_tuple_struct": {
      "title": "TupleStruct",
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
    },
    "prop_renamed_struct": {
      "title": "RenamedStruct",
      "type": "object",
      "properties": {
        "Value-Number": {
          "title": "i32",
          "type": "number"
        },
        "Value-String": {
          "title": "String",
          "type": "string"
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