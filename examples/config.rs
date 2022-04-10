#![allow(dead_code)]
#![allow(unused_imports)]

use rschema::{
    Schema,
    Schematic,
};
use serde::Deserialize;

use std::collections::HashMap;

#[derive(Debug, Schematic, Deserialize)]
#[serde(untagged)]
enum FullEnum {
    Unit,

    EmptyTuple(),

    SingleFieldTuple(bool),

    MultiFieldTuple(usize, String),

    #[rschema(
        additional_properties,
        rename_all = "kebab-case",
    )]
    Struct {
        #[rschema(
            title = "Number",
            description = "Number prop in enum's struct variant.",
        )]
        struct_variant_field: i32,
    },
}

#[derive(Debug, Schematic, Deserialize)]
struct UnitStruct;

#[derive(Debug, Schematic, Deserialize)]
struct EmptyTupleStruct();

#[derive(Debug, Schematic, Deserialize)]
struct SingleTupleStruct(String);

#[derive(Debug, Schematic, Deserialize)]
struct TupleStruct(usize, String);

#[derive(Debug, Schematic, Deserialize)]
struct EmptyStruct {}

#[derive(Debug, Schematic, Deserialize)]
struct NestedStruct {
    #[rschema(
        title = "Number",
        description = "This is number prop in nested-struct.",
    )]
    field_number: i32,
}

#[derive(Debug, Schematic, Deserialize)]
#[rschema(additional_properties)]
#[rschema(rename_all = "camelCase")]
struct Struct {
    #[rschema(
        title = "Number",
        description = "This is number prop in struct.",
        required,
    )]
    field_number: i32,

    #[rschema(
        title = "Number",
        description = "This is number prop in struct.",
    )]
    nested_struct: NestedStruct,
}

#[derive(Debug, Schematic, Deserialize)]
#[rschema(rename_all = "snake_case")]
enum UnitVariantOnlyEnum {
    UnitHoge,
    UnitFuga,
    UnitPiyo,
}

#[derive(Debug, Schematic, Deserialize)]
#[rschema(rename_all = "UPPERFLATCASE")]
pub struct Config {
    #[rschema(
        title = "String",
        description = "This is string.",
        min_length = 1,
        max_length = 15,
        pattern = r"^\w+$",
        format = "my format",
    )]
    prop_string: String,
    
    #[rschema(
        title = "Number (i8)",
        description = "This is number.",
        comment = "-128 .. 127",
        minimum = -128,
        maximum = 127,
    )]
    prop_number_i8: i8,
    
    #[rschema(
        title = "Number (i16)",
        description = "This is number.",
    )]
    prop_number_i16: i16,
    
    #[rschema(
        title = "Number (i32)",
        description = "This is number.",
        multiple_of = 5,
    )]
    prop_number_i32: i32,
    
    #[rschema(
        title = "Number (i64)",
        description = "This is number.",
    )]
    prop_number_i64: i64,
    
    #[rschema(
        title = "Number (isize)",
        description = "This is number.",
    )]
    prop_number_isize: isize,
    
    #[rschema(
        title = "Number (u8)",
        description = "This is number.",
        exclusive_minimum = 0,
        exclusive_maximum = 100,
    )]
    prop_number_u8: u8,
    
    #[rschema(
        title = "Number (u16)",
        description = "This is number.",
    )]
    prop_number_u16: u16,
    
    #[rschema(
        title = "Number (u32)",
        description = "This is number.",
    )]
    prop_number_u32: u32,
    
    #[rschema(
        title = "Number (u64)",
        description = "This is number.",
    )]
    prop_number_u64: u64,
    
    #[rschema(
        title = "Number (usize)",
        description = "This is number.",
    )]
    prop_number_usize: usize,
    
    #[rschema(
        title = "Boolean",
        description = "This is boolean.",
    )]
    prop_boolean: bool,
    
    #[rschema(
        title = "Optional",
        description = "This is optional value.",
    )]
    prop_optional: Option<String>,
    
    #[rschema(
        title = "Single-type array",
        description = "This is single-type array.",
        min_items = 1,
        max_items = 10,
        unique_items,
    )]
    prop_single_type_array: Vec<String>,
    
    #[rschema(
        title = "Composite-type array",
        description = "This is composite-type array.",
    )]
    prop_composite_type_array: Vec<FullEnum>,

    #[rschema(
        title = "Unit-struct",
        description = "This is unit-struct.",
    )]
    prop_unit_struct: UnitStruct, // => { "type": "null" }

    #[rschema(
        title = "Empty tuple-struct",
        description = "This is empty tuple-struct.",
    )]
    prop_empty_tuple_struct: EmptyTupleStruct,

    #[rschema(
        title = "Single tuple struct",
        description = "This is single tuple struct.",
    )]
    prop_single_tuple_struct: SingleTupleStruct,

    #[rschema(
        title = "Tuple struct",
        description = "This is tuple struct.",
    )]
    prop_tuple_struct: TupleStruct,

    #[rschema(
        title = "Empty struct",
        description = "This is empty struct.",
    )]
    prop_empty_struct: EmptyStruct,

    #[rschema(
        title = "Struct",
        description = "This is normal struct.",
    )]
    prop_struct: Struct,

    #[rschema(
        title = "Full enum",
        description = "This is full enum.",
    )]
    prop_full_enum: FullEnum,

    #[rschema(
        title = "Unit-variant-only enum",
        description = "This is unit-variant-only enum.",
    )]
    prop_unit_variant_only_enum: UnitVariantOnlyEnum,

    #[rschema(
        title = "Required prop",
        description = "This is required prop.",
        required,
    )]
    prop_required_prop: String,

    #[rschema(
        title = "Required prop 2",
        description = "This is another required prop.",
        required,
    )]
    prop_required_prop2: String,

    #[rschema(
        title = "Empty Tuple",
        description = "This is empty tuple.",
        deprecated,
    )]
    prop_empty_tuple: (),

    #[rschema(
        title = "One Member Tuple",
        description = "This is tuple with one member.",
    )]
    prop_one_member_tuple: (i8,),

    #[rschema(
        title = "Two Members Tuple",
        description = "This is tuple with two members.",
    )]
    prop_two_members_tuple: (i8, i16),

    #[rschema(
        title = "Three Members Tuple",
        description = "This is tuple with three members.",
    )]
    prop_three_members_tuple: (i32, i64, isize),

    #[rschema(
        title = "Twelve Members Tuple",
        description = "This is tuple with twelve members.",
    )]
    prop_twelve_members_tuple: (u8, u16, u32, u64, usize, f32, f64, bool, char, Option<String>, Vec<String>, Box<String>),

    #[rschema(
        title = "HashMap",
        description = "This is map with primitive type values",
    )]
    prop_hashmap_simple_values: HashMap<String, i32>,

    #[rschema(
        title = "HashMap",
        description = "This is map with object type values",
    )]
    prop_hashmap_object_values: HashMap<String, Struct>,

    #[rschema(
        title = "HashMap",
        description = "This is map with complex type values",
    )]
    prop_hashmap_complex_values: HashMap<String, FullEnum>,
}