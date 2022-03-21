#![allow(dead_code)]

use rschema::{
    Properties,
    Schema,
    Schematic,
};

#[derive(Debug)]
struct CustomString(String);

#[derive(Debug, Schematic)]
struct Config {
    #[rschema(
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

    #[rschema(
        field(
            title = "カスタム文字列型。",
            description = "カスタム文字列型です。",
            type = "string", // 明示的にstringと認識させる
        ),
        required,
    )]
    custom_str: CustomString,

    #[rschema(field(
        title = "データ",
        description = "データです。",
    ))]
    data: Data,
}

#[derive(Debug, Schematic)]
#[rschema(
    // title = "デフォルトのタイトルです。",
    // description = "デフォルトの説明です。",
    additional_properties,
)]
struct Data {
    #[rschema(field(
        title = "データサイズ。",
        description = "データサイズです。",
        minimum = 0,
        maximum = 100,
    ))]
    #[rschema(required)] // 分割してもよい
    size: i32,
}

fn main(){
    let schema = Schema::new::<Config>("MyConfig");

    // println!("{}", schema.to_string_pretty().unwrap());
    std::fs::write("schema.json", schema.to_string_pretty().unwrap()).unwrap();
}
