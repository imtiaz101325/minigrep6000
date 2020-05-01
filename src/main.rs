use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for \"{}\"", query);
    println!("Searching in ./{}", filename);

    let content = fs::read_to_string(filename)
        .expect("Something went wrong while trying to read the file");

    println!("File contains:\n{}", content);
}
