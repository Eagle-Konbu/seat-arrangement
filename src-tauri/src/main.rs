// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use solver::Student;
use tauri::{AppHandle, Window, WindowBuilder, WindowUrl};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct ExecutionResult {
    new_seat_assignment: Vec<Vec<Option<Student>>>,
    score: i64,
}

#[tauri::command]
fn solve(current_seat_assignment: Vec<Vec<Option<Student>>>) -> Result<ExecutionResult, String> {
    if current_seat_assignment
        .iter()
        .flatten()
        .all(|x| x.is_none())
    {
        return Err("席が空です。".to_string());
    }

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
        "current_seats",
        WindowUrl::App(format!("edit_layout?width={}&depth={}", width, depth).into()),
    )
    .title("現在の席配置")
    .resizable(true)
    .fullscreen(false)
    .build();

    match res {
        Ok(_) => {
            let close_res = window.close();
            match close_res {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Error closing window: {:?}", e)),
            }
        }
        Err(e) => Err(format!("Error opening window: {:?}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            solve,
            open_seats_edit_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
