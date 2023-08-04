#![feature(test)]
extern crate test;

mod beam_search;
mod eval_func;
mod simulated_annealing;
pub mod structs;
mod tabu_search;
mod utils;

use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::hash::{Hash, Hasher};
use std::{
    collections::{hash_map::DefaultHasher, BinaryHeap, HashSet, VecDeque},
    io::Error,
};

use structs::{SeatAssignment, Student};
use utils::{check_input, compress_student_id, separate_input};

pub fn solve(
    previous: &SeatAssignment,
    students: &[Student],
) -> Result<(SeatAssignment, i64), Error> {
    let seed = {
        let seed_base_str = format!(
            "{}{}",
            serde_json::to_string(students).unwrap(),
            serde_json::to_string(previous).unwrap()
        );

        let mut s = DefaultHasher::new();
        seed_base_str.hash(&mut s);
        s.finish()
    };

    let mut rng = ChaCha20Rng::seed_from_u64(seed);

    simulated_annealing::execute(previous, students, LOOP_CNT, &mut rng, T1, T2)
}

pub fn execute(
    current_layout: &[Vec<Option<Student>>],
) -> Result<(Vec<Vec<Option<Student>>>, i64), Error> {
    let check_res = check_input(current_layout);
    if check_res.is_err() {
        return Err(check_res.err().unwrap());
    }

    let (mut previous, mut students) = separate_input(current_layout);
    let original_student_ids = students.iter().map(|s| s.id).collect::<Vec<usize>>();

    compress_student_id(&mut students, &mut previous);

    let solve_result = solve(&previous, &students);

    if solve_result.is_err() {
        return Err(solve_result.err().unwrap());
    }

    let &score = &solve_result.as_ref().unwrap().1;

    let mut res = solve_result
        .unwrap()
        .0
        .iter()
        .map(|row| {
            row.iter()
                .map(|&idx| {
                    if idx == !0 {
                        None
                    } else {
                        Some(students[idx].clone())
                    }
                })
                .collect::<Vec<Option<Student>>>()
        })
        .collect::<Vec<Vec<Option<Student>>>>();

    for y in 0..res.len() {
        for x in 0..res[y].len() {
            if res[y][x].is_some() {
                res[y][x].as_mut().unwrap().id =
                    original_student_ids[res[y][x].as_ref().unwrap().id];
            }
        }
    }

    Ok((res, score))
}

const LOOP_CNT: usize = 200000;
const T1: f64 = 119.5;
const T2: f64 = 1.563;

#[cfg(test)]
mod tests {
    use crate::{
        structs::{Gender, Student},
        utils::swap_seats,
    };

    use super::*;

    #[test]
    fn check_ignoreing_vacant() {
        let students = (0..24)
            .map(|i| Student {
                id: i,
                name: format!("Student {}", i),
                academic_ability: 3,
                exercise_ability: 3,
                leadership_ability: 3,
                needs_assistance: i < 3,
                gender: if i < 15 { Gender::Male } else { Gender::Female },
            })
            .collect::<Vec<Student>>();

        let mut seat_assignment = vec![vec![!0; 5]; 5];
        for j in 0..24 {
            let (x, y) = (j % 5, j / 5);
            seat_assignment[y][x] = j;
        }
        for i in 0..25 {
            let (x, y) = (i % 5, i / 5);
            swap_seats(&mut seat_assignment, (x, y), (4, 4));
            let res = solve(&seat_assignment, &students);
            assert!(res.is_ok());
            swap_seats(&mut seat_assignment, (x, y), (4, 4));
        }
    }
}
