// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fileman;
// use crate::fileman::read_journals;
use std::io;
use std::fs;
use std::fs::read_to_string;

#[tauri::command]
fn test(){
    println!("hello!");
}

#[tauri::command]
fn read_journals(journalpath: &str) -> Vec<String> //reads dir only
{
    let mut result: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(journalpath) { //i have no clue why this works
        // Iterate over the directory entries
        for entry in entries {
            if let Ok(entry) = entry
            {
                // Get the file name as a string
                if let Some(file_name) = entry.file_name().to_str()
                {
                    // println!("{}", file_name);
                    result.push(String::from(file_name));
                }

                else
                {
                    eprintln!("Error converting file name to string");
                }
            }

            else
            {
                eprintln!("Error reading directory entry");
            }
        }
    }

    else
    {
        eprintln!("Error reading directory");
    }
    result
}


#[tauri::command]
fn read_contents(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn filesel(){
    println!("Please select a date : ");
    
    let files = read_journals("./data");
    
    for (index, file) in files.iter().enumerate() { //prints the list alongside the index for each
        println!("{}: {}", index, file);
    }

    let mut input = String::new(); // declares variable

    io::stdin().read_line(&mut input).expect("error");

    let num: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return; // Exit the program if input is invalid
        }
    };

    if num < files.len() {
        println!("You chose {}.", files[num])
    }
    else {
        println!("Element out of bounds.")
    }
}

// // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![test])
    .invoke_handler(tauri::generate_handler![read_journals])
    .invoke_handler(tauri::generate_handler![read_contents])
    .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
