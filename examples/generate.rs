use rschema::Schema;

mod config;
use config::Config;

fn main() -> rschema::Result<()> {
    let schema = Schema::new::<Config>("ExampleConfig");
    schema.write_pretty("examples/example.schema.json")?;

    Ok(())
}
