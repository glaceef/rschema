mod config;
use config::Config;

fn main() -> Result<(), serde_yaml::Error> {
    let config_yml = include_str!("config.yml");
    let config: Config = serde_yaml::from_str(&config_yml)?;
    println!("{:?}", config);

    Ok(())
}