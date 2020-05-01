use std::env;
use std::process;

use minigrep6000::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Probelm parsing argument: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep6000::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };
}
