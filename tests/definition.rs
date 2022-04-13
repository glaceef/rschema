#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
#[rschema(definition)]
struct Sample {
    prop_string: String,
}

mod external_crate {
    use super::*;

    #[derive(Debug, Schematic)]
    #[rschema(definition)]
    pub struct Sample {
        prop_string: String,
    }
}

#[derive(Debug, Schematic)]
struct Definition {
    prop_sample1: Sample,

    prop_sample2: external_crate::Sample,
}

#[test]
fn it_tests_definition() -> rschema::Result<()> {
    let schema_str = Schema::new::<Definition>("Definition")
        .to_string_pretty()?;
    let schema_str2 = r##"{
  "title": "Definition",
  "type": "object",
  "properties": {
    "prop_i32": {
      "type": "number"
    },
    "prop_struct1": {
      "$ref": "#/$defs/Struct1"
    },
    "prop_struct2": {
      "$ref": "#/$defs/Struct2"
    },
    "prop_nested": {
      "$ref": "#/$defs/NestedStruct"
    }
  },
  "additionalProperties": false,
  "$defs": {
    "Struct1": {
      "type": "object",
      "properties": {
        "prop_i32": {
          "type": "number"
        }
      },
      "additionalProperties": false
    },
    "Struct2": {
      "type": "object",
      "properties": {
        "prop_i32": {
          "type": "number"
        }
      },
      "additionalProperties": false
    },
    "NestedStruct": {
      "type": "object",
      "properties": {
        "prop_sample": {
          "$ref": "#/$defs/Sample"
        }
      },
      "additionalProperties": false
    },
    "Sample": {
      "type": "object",
      "properties": {
        "prop_string": {
          "type": "string"
        }
      },
      "additionalProperties": false
    }
  }
}"##;

    assert_eq!(schema_str, schema_str2);

    Ok(())
}