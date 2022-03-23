use rschema::Schema;

mod config;
use config::Config;

fn main() -> std::io::Result<()> {
    let schema = Schema::new::<Config>("ProjectConfig");
    schema.write_pretty("/path/to/rschema.schema.json")
}
