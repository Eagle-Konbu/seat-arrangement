// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use solver::Student;
use tauri::{AppHandle, Window, WindowBuilder, WindowUrl};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct ExecutionResult {
    new_seat_assignment: Vec<Vec<Option<Student>>>,
    score: i64,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn solve(current_seat_assignment: Vec<Vec<Option<Student>>>) -> Result<ExecutionResult, String> {
    let solver_res = solver::execute(&current_seat_assignment);

    if solver_res.is_err() {
        return Err(format!("Solver error: {:?}", solver_res.err()));
    }

    let (new_seat_assignment, score) = solver_res.unwrap();

    Ok(ExecutionResult {
        new_seat_assignment,
        score,
    })
}

#[tauri::command]
fn open_seats_edit_window(
    app: AppHandle,
    window: Window,
    width: usize,
    depth: usize,
) -> Result<(), String> {
    let res = WindowBuilder::new(
        &app,
        "現在の席配置",
        WindowUrl::App(format!("edit_layout?width={}&depth={}", width, depth).into()),
    )
    .title("Seats Layout")
    .resizable(true)
    .fullscreen(false)
    .build();

    match res {
        Ok(_) => {
            let _ = window.close();
            Ok(())
        }
        Err(e) => Err(format!("Error opening window: {:?}", e)),
    }
}

#[tauri::command]
fn open_result_window(app: AppHandle, result: ExecutionResult) -> Result<(), String> {
    let json_str = serde_json::to_string(&result).unwrap();
    let res = WindowBuilder::new(
        &app,
        "結果",
        WindowUrl::App(format!("result?result={}", json_str).into()),
    )
    .title("Result")
    .resizable(true)
    .fullscreen(false)
    .build();

    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error opening window: {:?}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            solve,
            open_seats_edit_window,
            open_result_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
