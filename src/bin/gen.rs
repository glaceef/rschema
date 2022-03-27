#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{
    Serialize,
    Deserialize,
};

use rschema::{
    Schema,
    Schematic,
};

#[derive(Debug, Schematic)]
struct NestedStruct {
    #[rschema(
        field(
            title = "数値",
            description = "数値です。",
        ),
        required
    )]
    field_number: i32,
}

#[derive(Debug, Schematic)]
struct Struct {
    #[rschema(
        field(
            title = "文字列",
            description = "文字列です。",
        ),
        required
    )]
    field_string: String,

    #[rschema(
        field(
            title = "ネストした構造体",
            description = "ネストした構造体です。",
        ),
        required
    )]
    field_nested: NestedStruct,
}

#[derive(Debug, Schematic)]
struct NewTypeStruct(String);

#[derive(Debug, Schematic)]
struct TupleStruct(usize, String);

#[derive(Debug, Schematic)]
enum Enum {
    // これどーしよ
    // Unit,

    Struct {
        #[rschema(field(
            title = "aaa",
            description = "bbb",
        ))]
        value: i32,
    },
    NewType(String),
    Tuple(usize, String),
}

#[derive(Debug, Schematic)]
struct Data {
    #[rschema(
        field(
            title = "文字列",
            description = "文字列です。",
            min_length = 1,
            max_length = 15,
            pattern = r"^\w+$",
            // format
        ),
        required
    )]
    prop_string: String,
    
    #[rschema(
        field(
            title = "数値",
            description = "数値です。",
            minimum = 1,
            maximum = 100,
            multiple_of = 5,
            exclusive_minimum = true,
            exclusive_maximum = false,
        ),
        required
    )]
    prop_number: i32, // TODO: 全数値型に対して行う
    
    #[rschema(
        field(
            title = "真偽値",
            description = "真偽値です。",
        ),
        required
    )]
    prop_boolean: bool,
    
    #[rschema(field(
        title = "たぶん文字列",
        description = "たぶん文字列です。",
    ))]
    prop_oprional: Option<String>,
    
    #[rschema(
        field(
            title = "文字列配列",
            description = "文字列配列です。",
        ),
        required
    )]
    prop_array: Vec<String>,
    
    #[rschema(
        field(
            title = "構造体",
            description = "構造体です。",
        ),
        required
    )]
    prop_struct: Struct,
    
    #[rschema(
        field(
            title = "NewType構造体",
            description = "NewType構造体です。",
        ),
        required
    )]
    prop_newtype: NewTypeStruct,
    
    #[rschema(
        field(
            title = "タプル構造体",
            description = "タプル構造体です。",
        ),
        required
    )]
    prop_tuple: TupleStruct,

    #[rschema(
        field(
            title = "列挙型",
            description = "列挙型です。",
        ),
        required
    )]
    prop_enum: Enum,

    #[rschema(
        field(
            title = "列挙型配列",
            description = "列挙型配列です。",
        ),
        required
    )]
    prop_enum_array: Vec<Enum>,

    #[rschema(field(
        title = "requiredでないフィールド",
        description = "requiredでないフィールドです。",
    ))]
    prop_unrequired: String,
}

fn main(){
    let schema = Schema::new::<Data, _>("データ");
    // println!("{:#?}", schema);

    let schema_str = serde_json::to_string_pretty(&schema).unwrap();
    println!("{}", schema_str);
}
