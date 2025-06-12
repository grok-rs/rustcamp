use anyhow::{anyhow, Result};
use clap::Parser;
use std::borrow::Cow;

const DEFAULT_PATH: &str = "/etc/app/app.conf";

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short = 'c', long = "conf")]
    conf: Option<String>,
}

fn get_config_path(conf_arg: Option<String>, env_var: Option<String>) -> Result<Cow<'static, str>> {
    if let Some(conf) = conf_arg {
        return if conf.is_empty() {
            Err(anyhow!("The -c/--conf argument cannot be empty"))
        } else {
            Ok(Cow::Owned(conf))
        };
    }

    if let Some(app_conf) = env_var {
        if !app_conf.is_empty() {
            return Ok(Cow::Owned(app_conf));
        }
    }

    Ok(Cow::Borrowed(DEFAULT_PATH))
}

fn main() -> Result<()> {
    let args = Args::parse();
    let env_conf = std::env::var("APP_CONF").ok();
    let path = get_config_path(args.conf, env_conf)?;
    println!("{}", path);
    Ok(())
}
