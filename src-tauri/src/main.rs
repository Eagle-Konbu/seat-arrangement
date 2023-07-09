// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use solver::Student;
use std::io::Error;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn solve(current_seat_assignment: Vec<Vec<Student>>) -> Result<(Vec<Vec<Student>>, i64), String> {
    Err("Not implemented!".into())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, solve])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
