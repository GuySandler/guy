use std::env;
use std::fs;
use std::process::exit;
// use crossterm::event::{read, Event, KeyCode, KeyEvent};
use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    // lists of menu
    let mut dirs: Vec<String> = vec![];
    let mut files: Vec<String> = vec![];
    // menu
    let mut selection: &Vec<String> = &vec![];
    let mut menu: &Vec<String> = &vec![];
    let mut selected: usize = 0;

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
            println!("⇦: ..");
            for item in dirs {
                // println!("📁: {}", item);
                menu.push(format!("📁: {}", item))
            }
        }
        if args[1] == "fil" {
            println!("⇦: ..");
            for item in files
            {
                // println!("📄: {}", item);
                menu.push(format!("📄: {}", item))
            }
        }
    }
    // no command
    else {
        println!("⇦: ..");
        for item in dirs {
            // println!("📁: {}", item);
            menu.push(format!("📁: {}", item))
        }
        for item in files {
            // println!("📄: {}", item);
            menu.push(format!("📄: {}", item))
        }
    }
    // keyboard output format
    // loop {
        menu = print_menu(menu, selected);
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your flavor")
            .default(0)
            .items(&menu[..])
            .interact()
            .unwrap();
        println!("Enjoy your {}!", menu[selection]);
    // }
}
fn print_menu(menu: &Vec<String>, selected: usize) -> &Vec<String> {
    // function print_menu(menu)
    for (i, item) in menu.iter().enumerate() {
        if i == selected {
            println!("> {}", item);
        } else {
            println!("  {}", item);
        }
    }
    menu
}