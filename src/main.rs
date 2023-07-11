use std::env;
use std::fs;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "init" {
        fs::create_dir(".cr").unwrap();
        fs::File::create(".cr/stagged").unwrap();
    }

    if command == "add" {
        let dir = fs::read_dir(".");

        let entries = match dir {
            Ok(entries) => entries,
            Err(_) => panic!("Error reading directory"),
        };

        let mut files: Vec<String> = Vec::new();

        for entry in entries {
            let entry = entry.unwrap();
            println!("{}", entry.file_name().into_string().unwrap());
            files.push(entry.file_name().into_string().unwrap());
        }

        let stagged_files = files.join(",");
        let write_status = fs::write(".cr/stagged", stagged_files);

        match write_status {
            Ok(_) => println!("Files added to stagged"),
            Err(_) => panic!("Error writing stagged file"),
        }
    }
}
