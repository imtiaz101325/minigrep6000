use std::env;
use std::process;

use minigrep6000::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Probelm parsing argument: {}", err);
        process::exit(1);
    });

    println!("Searching for \"{}\"", config.query);
    println!("Searching in ./{}", config.filename);

    if let Err(e) = minigrep6000::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    };
}
