use std::env;
// use std::path::{PathBuf};
use std::fs;
use std::process::exit;

// use std::fs::ReadDir;
// use colored::Colorize;
fn main() {
    // lists of items
    let mut dirs: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];

    // getting files in dir
    let current_dir = env::current_dir().unwrap();
    println!("Current directory: {}", current_dir.display());
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        if let Ok(entry) = path {
            if let Some(file_name) = entry.path().file_name() {
                if entry.file_type().unwrap().is_file() {
                    files.push(file_name.to_string_lossy().to_string());
                } else if entry.file_type().unwrap().is_dir() {
                    dirs.push(file_name.to_string_lossy().to_string());
                }
            }
        }
    }

    // format output
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 {
        // check args
        if args[1] == "dir" || args[1] == "fil" {} else {
            println!("command not found");
            exit(1);
        }

        // commands
        if args[1] == "dir" {
            for item in dirs {
                println!("📁: {}", item);
            }
        }
        if args[1] == "fil" {
            for item in files
            {
                println!("📄: {}", item);
            }
        }
    }
    else {
        for item in dirs {
            println!("📁: {}", item);
        }
        for item in files {
            println!("📄: {}", item);
        }
    }
}
