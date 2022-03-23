#![allow(dead_code)]

use rschema::Schematic;
use serde::Deserialize;

#[derive(Debug, Deserialize, Schematic)]
pub struct Config {
    #[rschema(
        field(
            // `title` and `description` are used for all types.
            title = "project name", // required
            description = "This is project name.", // optional
        ),
        required,
    )]
    name: String,

    #[rschema(field(
        title = "version",
        description = "This is version.",

        // `pattern` is allowed for string type only.
        // Write in regular expression format.
        pattern = "^[0-9]+\\.[0-9]+\\.[0-9]+$",
    ))]
    #[rschema(required)] // Ok to write separately.
    version: String,

    #[rschema(field(
        title = "project members",
        description = "They are project members. Up to 10 members.",
        type = "array",
        max_length = 10,
    ))]
    members: Vec<String>,

    // Custom data types are interpreted as object type.
    // The type must derives `Schematic`.
    #[rschema(field(
        title = "data",
        description = "This is a data.",
    ))]
    data: Data,
}

#[derive(Debug, Deserialize, Schematic)]
#[rschema(additional_properties)]
struct Data {
    #[rschema(field(
        title = "number",
        description = "This is numeric value between 0 and 100.",
        minimum = 0,
        maximum = 100,
    ))]
    data1: i32,

    #[rschema(field(
        title = "custom string",
        description = "This is custom string value.",
        type = "string", // Specify type
    ))]
    data2: CustomString,

    #[rschema(field(
        title = "boolean",
        description = "This is boolean value.",
    ))]
    data3: bool,
}

#[derive(Debug, Deserialize)]
struct CustomString(String);
