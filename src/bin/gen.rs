#![allow(dead_code)]

use json_schema_gen::{
    Properties,
    Schema,
    SchemaBuilder,
    ToProperties,
};

#[derive(Debug)]
struct CustomString(String);

#[derive(Debug, Schema)]
struct Config {
    #[field(
        title = "バージョン。",
        description = "バージョンです。",

        min_length = 0,
        max_length = 20,
        pattern = "^[0-9]+\\.[0-9]+\\.[0-9]+$"
    )]
    version: String,
    // いつかこの String を置き換えるやり方も試してみたい。
    // Deref, DerefMut を実装したらまったく同じように動かせるだろうか？

    #[field(
        title = "カスタム文字列型。",
        description = "カスタム文字列型です。",
        type = "string", // 明示的にstringと認識させる
    )]
    custom_str: CustomString,

    #[field(
        title = "データ",
        description = "データです。",
    )]
    data: Data,
}

#[derive(Debug, Schema)]
struct Data {
    #[field(
        title = "データサイズ。",
        description = "データサイズです。",
        minimum = 0,
        maximum = 100,
    )]
    value: i32,
}

fn main(){
    // let s = <Config as ToProperties>::PROPERTIES_STR;
    // println!("{}", s);

    let schema: Schema = SchemaBuilder::new("MyConfig")
        .required(vec!["required_prop".into()])
        .additional_properties(false)
        .build::<Config>()
        .unwrap();
    println!("{}", schema.to_string_pretty().unwrap());
    // std::fs::write("schema.json", schema.to_string_pretty().unwrap());
}
