#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

mod external_crate {
    use super::*;

    #[derive(Debug, Schematic)]
    #[rschema(definitions)]
    pub struct Struct {
        prop_value: i32,
    }

    #[derive(Debug, Schematic)]
    #[rschema(definitions)]
    pub struct Tuple(i32, String);
}

#[derive(Debug, Schematic)]
struct NoDefStruct {
    prop_value: i32,

    prop_struct: Struct,
}

#[derive(Debug, Schematic)]
#[rschema(definitions)]
struct Struct {
    prop_value: i32,
}

#[derive(Debug, Schematic)]
#[rschema(definitions)]
struct NestedStruct {
    prop_struct: Struct,
}

#[derive(Debug, Schematic)]
#[rschema(definitions)]
struct Alt2 {
    prop_value: i32,
}

#[derive(Debug)]
struct Alt {
    prop_value: i32,
}

#[derive(Debug, Schematic)]
#[rschema(definitions)]
struct NewTypeStruct(
    #[rschema(minimum = 0)]
    i32
);

#[derive(Debug, Schematic)]
struct NoDefTuple(i32, String);

#[derive(Debug, Schematic)]
#[rschema(definitions)]
struct Tuple(i32, String);

#[derive(Debug, Schematic)]
#[rschema(definitions)]
struct NestedTuple(i32, Tuple);

#[derive(Debug, Schematic)]
#[rschema(definitions)]
enum Enum {
    Tuple(i32),

    Struct {
        prop_struct: Struct,
        prop_tuple: Tuple,
    },
}

#[derive(Debug, Schematic)]
struct Definitions {
    prop_no_def_struct: NoDefStruct,

    prop_struct: Struct,

    prop_struct_external: external_crate::Struct,

    prop_nested_struct: NestedStruct,

    #[rschema(alt = "Alt2")]
    prop_alt: Alt,

    prop_new_type: NewTypeStruct,

    prop_no_def_tuple: NoDefTuple,

    prop_tuple: Tuple,

    prop_tuple_external: external_crate::Tuple,

    prop_nexted_tuple: NestedTuple,

    prop_enum: Enum,
}

#[test]
fn it_tests_definitions() -> rschema::Result<()> {
    let schema_str = Schema::new::<Definitions>("Definitions")
        .to_string_pretty()?;
    let schema_str2 = r##"{
  "title": "Definitions",
  "type": "object",
  "properties": {
    "prop_no_def_struct": {
      "type": "object",
      "properties": {
        "prop_value": {
          "type": "number"
        },
        "prop_struct": {
          "$ref": "#/$defs/definitions::Struct"
        }
      },
      "additionalProperties": false
    },
    "prop_struct": {
      "$ref": "#/$defs/definitions::Struct"
    },
    "prop_struct_external": {
      "$ref": "#/$defs/definitions::external_crate::Struct"
    },
    "prop_nested_struct": {
      "$ref": "#/$defs/definitions::NestedStruct"
    }
  },
  "additionalProperties": false,
  "$defs": {
    "definitions::Struct": {
      "type": "object",
      "properties": {
        "prop_value": {
          "type": "number"
        }
      },
      "additionalProperties": false
    },
    "definitions::external_crate::Struct": {
      "type": "object",
      "properties": {
        "prop_value": {
          "type": "number"
        }
      },
      "additionalProperties": false
    },
    "definitions::NestedStruct": {
      "type": "object",
      "properties": {
        "prop_struct": {
          "$ref": "#/$defs/definitions::Struct"
        }
      },
      "additionalProperties": false
    }
  }
}"##;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}