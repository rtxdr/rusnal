// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fileman;
use crate::fileman::read_journals;
use std::io;

#[tauri::command]
fn test(){
    println!("hello!");
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
