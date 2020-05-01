use std::env;
use std::process;

use minigrep6000::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Probelm parsing argument: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep6000::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };
}
