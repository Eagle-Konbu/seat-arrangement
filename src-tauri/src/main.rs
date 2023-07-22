// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use solver::Student;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

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
fn gen_pdf_bytes(seat_assignment: Vec<Vec<Option<Student>>>) -> Result<Vec<u8>, String> {
    let seats = seat_assignment
        .iter()
        .map(|row| {
            row.iter()
                .map(|seat| {
                    if let Some(student) = seat {
                        format!("{}. {}", student.id, &student.name)
                    } else {
                        "".to_string()
                    }
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let bytes = pdf_generator::gen(seats);

    if bytes.is_err() {
        return Err(format!("PDF generator error: {:?}", bytes.err()));
    }

    Ok(bytes.unwrap())
}

fn main() {
    let save = CustomMenuItem::new("save".to_string(), "Save").accelerator("Ctrl+S");
    let open = CustomMenuItem::new("open".to_string(), "Open").accelerator("Ctrl+O");
    let file = Submenu::new("File", Menu::new().add_item(save).add_item(open));

    let change_size = CustomMenuItem::new("change_size".to_string(), "Change size");
    let edit = Submenu::new("Edit", Menu::new().add_item(change_size));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_submenu(file)
        .add_submenu(edit);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "change_size" => {
                let window = event.window();
                let _ = window.emit("change_size", "change_size".to_string());
            }
            "save" => {
                let window = event.window();
                let _ = window.emit("save", "save".to_string());
            }
            "open" => {
                let window = event.window();
                let _ = window.emit("open", "open".to_string());
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![solve, gen_pdf_bytes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
