use std::env;
// use std::path::{PathBuf};
use std::fs;
// use std::fs::ReadDir;
// use colored::Colorize;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let arg: &String = &args[1];
    // println!("{}", arg);

    // lists of items
    let mut dirs : Vec<String> = vec![];
    let mut files : Vec<String> = vec![];

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
    // println!("{:?}", dirs);
    // println!("{:?}", files);
    // format output
    for item in dirs{
        println!("📁: {}",item);
    }
    for item in files{
        println!("📄: {}",item);
    }
    // end
    println!();
}