// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use solver::Student;
use tauri::{AppHandle, WindowBuilder, WindowUrl};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn validate_check_for_seat_assignment(
    current_seat_assignment: &[Vec<Option<Student>>],
) -> Result<(), String> {
    let mut student_ids = current_seat_assignment
        .iter()
        .flatten()
        .filter_map(|s| s.as_ref())
        .map(|s| s.id)
        .collect::<Vec<usize>>();
    student_ids.sort();

    let missing_ids = (1..=student_ids.len())
        .filter(|&i| i != student_ids[i])
        .collect::<Vec<usize>>();
    if !missing_ids.is_empty() {
        return Err(format!(
            "Missing student ids: {:?}",
            missing_ids
                .iter()
                .map(|&i| i.to_string())
                .collect::<Vec<String>>()
        ));
    }

    Ok(())
}

#[tauri::command]
fn solve(
    current_seat_assignment: Vec<Vec<Option<Student>>>,
) -> Result<(Vec<Vec<Option<Student>>>, i64), String> {
    let validate_check_res = validate_check_for_seat_assignment(&current_seat_assignment);
    validate_check_res?;

    let mut idx_seat_assignment = current_seat_assignment
        .iter()
        .map(|row| {
            row.iter()
                .map(|student| {
                    if student.is_none() {
                        !0
                    } else {
                        student.as_ref().unwrap().id
                    }
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut students = vec![];
    for y in 0..idx_seat_assignment.len() {
        for x in 0..idx_seat_assignment[y].len() {
            if idx_seat_assignment[y][x] != !0 {
                students.push(current_seat_assignment[y][x].as_ref().unwrap().clone());
            }
        }
    }

    for y in 0..idx_seat_assignment.len() {
        for x in 0..idx_seat_assignment[y].len() {
            idx_seat_assignment[y][x] -= 1;
        }
    }

    students.sort_by_key(|s| s.id);
    for s in students.iter_mut() {
        s.id -= 1;
    }

    let solver_res = solver::solve(&idx_seat_assignment, &students);
    if let Err(e) = solver_res {
        return Err(format!("Solver error: {:?}", e));
    }

    let (new_idx_seat_assignment, score) = solver_res.unwrap();

    let mut new_seat_assignment = vec![];
    for y in 0..new_idx_seat_assignment.len() {
        new_seat_assignment.push(vec![None; new_idx_seat_assignment[y].len()]);
        for x in 0..new_idx_seat_assignment[y].len() {
            if new_idx_seat_assignment[y][x] != !0 {
                new_seat_assignment[y][x] = Some(students[new_idx_seat_assignment[y][x]].clone());
            }
        }
    }

    Ok((new_seat_assignment, score))
}

#[tauri::command]
fn open_seats_edit_window(app: AppHandle, width: usize, depth: usize) -> Result<(), String> {
    let res = WindowBuilder::new(
        &app,
        "seats_layout",
        WindowUrl::App(format!("edit_layout?width={}&depth={}", width, depth).into()),
    )
    .title("Seats Layout")
    .resizable(true)
    .fullscreen(false)
    .build();

    println!("width: {}, depth: {}", width, depth);

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
            open_seats_edit_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
