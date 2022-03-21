#![allow(dead_code)]

use json_schema_gen::{
    Properties,
    Schema,
    ToProperties,
};

#[derive(Debug)]
struct CustomString(String);

#[derive(Debug, Schema)]
struct Config {
    #[schema(
        field(
            title = "バージョン。",
            description = "バージョンです。",

            min_length = 0,
            max_length = 20,
            pattern = "^[0-9]+\\.[0-9]+\\.[0-9]+$"
        ),
        required = true,
    )]
    version: String,

    #[schema(
        field(
            title = "カスタム文字列型。",
            description = "カスタム文字列型です。",
            type = "string", // 明示的にstringと認識させる
        ),
        required,
    )]
    custom_str: CustomString,

    #[schema(field(
        title = "データ",
        description = "データです。",
    ))]
    data: Data,
}

#[derive(Debug, Schema)]
#[schema(additional_properties)]
struct Data {
    #[schema(field(
        title = "データサイズ。",
        description = "データサイズです。",
        minimum = 0,
        maximum = 100,
    ))]
    #[schema(required)] // 分割してもよい
    size: i32,
}

fn main(){
    let schema = Schema::new::<Config>("MyConfig");

    // println!("{}", schema.to_string_pretty().unwrap());
    std::fs::write("schema.json", schema.to_string_pretty().unwrap()).unwrap();
}
