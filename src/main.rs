use std::env;
use std::fs;
use std::process::exit;
use dialoguer::{theme::ColorfulTheme, Select};
fn main() {
    // lists of menu
    let mut dirs: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];
    // menu
    let mut menu: Vec<String> = vec!["🚪: exit".to_string(), "⇦: ..".to_string()];
    let mut ui: bool = true;

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
        if args[1] == "dir" || args[1] == "fil" || args[1] == "noui" {} else {
            println!("command not found");
            exit(1);
        }

        // commands
        if args[1] == "dir" {
            for item in dirs {
                menu.push(format!("📁: {}", item))
            }
        }
        else if args[1] == "fil" {
            for item in files
            {
                menu.push(format!("📄: {}", item))
            }
        }
        else if args[1] == "noui" {
            ui = false;
            for item in dirs {
                menu.push(format!("📁: {}", item))
            }
            for item in files {
                menu.push(format!("📄: {}", item))
            }
        }
    }
    // no command
    else {
        for item in dirs {
            menu.push(format!("📁: {}", item))
        }
        for item in files {
            menu.push(format!("📄: {}", item))
        }
    }
    // output format
    if ui == true {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your flavor")
            .default(0)
            .items(&menu[..])
            .interact()
            .unwrap();
        println!("Enjoy your {}!", menu[selection]);
    }
    else if ui == false {
        menu.remove(0);
        menu.remove(0);
        for item in menu {
            println!("{}", item)
        }
    }
}