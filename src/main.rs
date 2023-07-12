use std::env;
use std::fs;

const IGNORED_FILES: [&str; 3] = [".cr", ".git", ".gitignore"];
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

            let filename = entry.file_name().into_string().unwrap();
            if IGNORED_FILES.contains(&filename.as_str()) {
                continue;
            }

            files.push(filename);
        }

        let stagged_files = files.join(",");
        let write_status = fs::write(".cr/stagged", stagged_files);

        match write_status {
            Ok(_) => println!("Files added to stagged"),
            Err(_) => panic!("Error writing stagged file"),
        }
    }
}
