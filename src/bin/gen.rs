#![allow(dead_code)]

use rschema::{
    Schema,
    Schematic,
};

mod data {
    use rschema::Schematic;

    #[derive(Debug, Schematic)]
    #[rschema(
        // title = "デフォルトのタイトルです。",
        // description = "デフォルトの説明です。",
        additional_properties,
    )]
    pub struct Data<T> {
        #[rschema(field(
            title = "データサイズ。",
            description = "データサイズです。",
            minimum = 0,
            maximum = 100,
        ))]
        #[rschema(required)] // 分割してもよい
        size: i32,

        // #[rschema(required)]
        phantom: std::marker::PhantomData<T>,
    }
}

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
            pattern = "^[0-9]+\\.[0-9]+\\.[0-9]+$",
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
    data: data::Data<String>,

    // #[rschema(field(
    //     title = "タプル構造体。",
    //     description = "タプル構造体です。",
    // ))]
    // tdata: TupleData,
}

// #[derive(Debug, Schematic)]
// struct TupleData(
//     #[rschema(field(
//         title = "タプル構造体。",
//         description = "タプル構造体です。",
//     ))]
//     String
// );

fn main(){
    let schema = Schema::new::<Config>("MyConfig");
    schema.write_pretty("schema.json").unwrap();
}
