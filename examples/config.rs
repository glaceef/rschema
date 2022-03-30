#![allow(dead_code)]
#![allow(unused_imports)]

use rschema::{
    Schema,
    Schematic,
};
use serde::Deserialize;

#[derive(Debug, Schematic, Deserialize)]
#[serde(untagged)]
enum FullEnum {
    Unit,

    EmptyTuple(),

    SingleFieldTuple(bool),

    MultiFieldTuple(usize, String),

    #[rschema(additional_properties)]
    Struct {
        #[rschema(field(
            title = "Number",
            description = "Number prop in enum's struct variant.",
        ))]
        field: i32,
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
    #[rschema(field(
        title = "Number",
        description = "This is number prop in nested-struct.",
    ))]
    field_number: i32,
}

#[derive(Debug, Schematic, Deserialize)]
#[rschema(additional_properties)]
struct Struct {
    #[rschema(
        field(
            title = "Number",
            description = "This is number prop in struct.",
        ),
        required,
    )]
    field_number: i32,

    #[rschema(field(
        title = "Number",
        description = "This is number prop in struct.",
    ))]
    nested_struct: NestedStruct,
}

#[derive(Debug, Schematic, Deserialize)]
enum UnitVariantOnlyEnum {
    Unit1,
    Unit2,
    Unit3,
}

#[derive(Debug, Schematic, Deserialize)]
pub struct Config {
    #[rschema(field(
        title = "String",
        description = "This is string.",
        min_length = 1,
        max_length = 15,
        pattern = r"^\w+$",
        format = "my format",
    ))]
    prop_string: String,
    
    #[rschema(field(
        title = "Number (i8)",
        description = "This is number.",
        minimum = 0,
        maximum = 256,
        exclusive_minimum = false,
        exclusive_maximum = true,
    ))]
    prop_number_i8: i8,
    
    #[rschema(field(
        title = "Number (i16)",
        description = "This is number.",
    ))]
    prop_number_i16: i16,
    
    #[rschema(field(
        title = "Number (i32)",
        description = "This is number.",
        multiple_of = 5,
    ))]
    prop_number_i32: i32,
    
    #[rschema(field(
        title = "Number (i64)",
        description = "This is number.",
    ))]
    prop_number_i64: i64,
    
    #[rschema(field(
        title = "Number (isize)",
        description = "This is number.",
    ))]
    prop_number_isize: isize,
    
    #[rschema(field(
        title = "Number (u8)",
        description = "This is number.",
    ))]
    prop_number_u8: u8,
    
    #[rschema(field(
        title = "Number (u16)",
        description = "This is number.",
    ))]
    prop_number_u16: u16,
    
    #[rschema(field(
        title = "Number (u32)",
        description = "This is number.",
    ))]
    prop_number_u32: u32,
    
    #[rschema(field(
        title = "Number (u64)",
        description = "This is number.",
    ))]
    prop_number_u64: u64,
    
    #[rschema(field(
        title = "Number (usize)",
        description = "This is number.",
    ))]
    prop_number_usize: usize,
    
    #[rschema(field(
        title = "Boolean",
        description = "This is boolean.",
    ))]
    prop_boolean: bool,
    
    #[rschema(field(
        title = "Optional",
        description = "This is optional value.",
    ))]
    prop_optional: Option<String>,
    
    #[rschema(field(
        title = "Single-type array",
        description = "This is single-type array.",
        min_items = 1,
        max_items = 10,
    ))]
    prop_single_type_array: Vec<String>,
    
    #[rschema(field(
        title = "Composite-type array",
        description = "This is composite-type array.",
    ))]
    prop_composite_type_array: Vec<FullEnum>,

    #[rschema(field(
        title = "Unit-struct",
        description = "This is unit-struct.",
    ))]
    prop_unit_struct: UnitStruct, // => { "type": "null" }

    #[rschema(field(
        title = "Empty tuple-struct",
        description = "This is empty tuple-struct.",
    ))]
    prop_empty_tuple_struct: EmptyTupleStruct,

    #[rschema(field(
        title = "Single tuple struct",
        description = "This is single tuple struct.",
    ))]
    prop_single_tuple_struct: SingleTupleStruct,

    #[rschema(field(
        title = "Tuple struct",
        description = "This is tuple struct.",
    ))]
    prop_tuple_struct: TupleStruct,

    #[rschema(field(
        title = "Empty struct",
        description = "This is empty struct.",
    ))]
    prop_empty_struct: EmptyStruct,

    #[rschema(field(
        title = "Struct",
        description = "This is normal struct.",
    ))]
    prop_struct: Struct,

    #[rschema(field(
        title = "Full enum",
        description = "This is full enum.",
    ))]
    prop_full_enum: FullEnum,

    #[rschema(field(
        title = "Unit-variant-only enum",
        description = "This is unit-variant-only enum.",
    ))]
    prop_unit_variant_only_enum: UnitVariantOnlyEnum,

    #[rschema(
        field(
            title = "Required prop",
            description = "This is required prop.",
        ),
        required,
    )]
    prop_required_prop: String,

    #[rschema(
        field(
            title = "Required prop 2",
            description = "This is another required prop.",
        ),
        required,
    )]
    prop_required_prop2: String,

    #[rschema(field(
        title = "Empty Tuple",
        description = "This is empty tuple.",
    ))]
    prop_empty_tuple: (),

    #[rschema(field(
        title = "One Member Tuple",
        description = "This is tuple with one member.",
    ))]
    prop_one_member_tuple: (i8,),

    #[rschema(field(
        title = "Two Members Tuple",
        description = "This is tuple with two members.",
    ))]
    prop_two_members_tuple: (i8, i16),

    #[rschema(field(
        title = "Three Members Tuple",
        description = "This is tuple with three members.",
    ))]
    prop_three_members_tuple: (i32, i64, isize),

    #[rschema(field(
        title = "Twelve Members Tuple",
        description = "This is tuple with twelve members.",
    ))]
    prop_twelve_members_tuple: (u8, u16, u32, u64, usize, f32, f64, bool, char, Option<String>, Vec<String>, Box<String>),
}