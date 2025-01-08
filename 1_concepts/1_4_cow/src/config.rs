use clap::{Arg, Command};
use std::borrow::Cow;

const DEFAULT_CONF_PATH: &str = "config.toml";

fn parse_args() -> Option<String> {
    let matches = Command::new("Config path detector")
        .arg(
            Arg::new("conf")
                .short('c')
                .long("conf")
                .value_name("FILE")
                .help("Path to the configuration file"),
        )
        .get_matches();

    matches.get_one::<String>("conf").cloned()
}

fn get_env_conf() -> Option<String> {
    std::env::var("CONF_PATH").ok()
}

fn validate_conf_path(path: &str) -> Result<(), String> {
    if path.is_empty() {
        Err("Configuration file path cannot be empty".to_string())
    } else {
        Ok(())
    }
}

pub fn get_conf_path() -> Result<Cow<'static, str>, String> {
    if let Some(conf_path) = parse_args() {
        validate_conf_path(&conf_path)?;
        Ok(Cow::Owned(conf_path))
    } else if let Some(env_path) = get_env_conf() {
        validate_conf_path(&env_path)?;
        Ok(Cow::Owned(env_path))
    } else {
        Ok(Cow::Borrowed(DEFAULT_CONF_PATH))
    }
}
