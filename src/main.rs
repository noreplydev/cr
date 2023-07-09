use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "init" {
        fs::create_dir(".cr").unwrap();
    }
}
