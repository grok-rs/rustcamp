mod config;

fn main() {
    match config::get_conf_path() {
        Ok(conf_path) => println!("Configuration file path: {}", conf_path),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
