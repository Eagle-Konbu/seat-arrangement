use rand::seq::SliceRandom;
use rand_chacha::ChaCha20Rng;
use rand_distr::{Distribution, Normal};

use crate::structs::{Gender, SeatAssignment, Student};

use std::{collections::HashSet, io::Error};

pub fn separate_input(input: &[Vec<Option<Student>>]) -> (SeatAssignment, Vec<Student>) {
    let idx_seat_assignment = input
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
                students.push(input[y][x].as_ref().unwrap().clone());
            }
        }
    }
    students.sort_by_key(|s| s.id);

    (idx_seat_assignment, students)
}

pub fn check_input(input: &[Vec<Option<Student>>]) -> Result<(), Error> {
    let studnet_ids = input
        .iter()
        .flatten()
        .filter(|s| s.is_some())
        .map(|s| s.as_ref().unwrap().id)
        .collect::<Vec<usize>>();

    let mut id_set = HashSet::new();
    let mut duplicated_ids = vec![];

    for &id in studnet_ids.iter() {
        if id_set.contains(&id) {
            duplicated_ids.push(id);
        }
        id_set.insert(id);
    }

    duplicated_ids.dedup();
    duplicated_ids.sort();

    if !duplicated_ids.is_empty() {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Duplicated student ids: {:?}", duplicated_ids),
        ));
    }

    Ok(())
}

pub fn compress_student_id(students: &mut [Student], idx_layout: &mut SeatAssignment) {
    let student_ids = students.iter().map(|s| s.id).collect::<Vec<usize>>();

    let sorted_student_ids = {
        let mut ids = student_ids.clone();
        ids.sort();
        ids
    };

    for student in students.iter_mut() {
        let idx = sorted_student_ids
            .binary_search(&student.id)
            .expect("Student id not found in sorted ids");

        student.id = idx;
    }

    students.sort_by_key(|s| s.id);

    for y in 0..idx_layout.len() {
        for x in 0..idx_layout[y].len() {
            if idx_layout[y][x] != !0 {
                let idx = student_ids
                    .binary_search(&idx_layout[y][x])
                    .expect("Student id not found in sorted ids");

                idx_layout[y][x] = idx;
            }
        }
    }
}

pub fn swap_seats(assigment: &mut SeatAssignment, pos1: (usize, usize), pos2: (usize, usize)) {
    let tmp = assigment[pos1.1][pos1.0];
    assigment[pos1.1][pos1.0] = assigment[pos2.1][pos2.0];
    assigment[pos2.1][pos2.0] = tmp;
}

pub fn test_case(rng: &mut ChaCha20Rng) -> (SeatAssignment, Vec<Student>) {
    let normal = Normal::<f64>::new(3.0, 1.0).unwrap();

    let students = (0..30)
        .map(|i| Student {
            id: i,
            name: format!("Student {}", i),
            academic_ability: (normal.sample(rng).round() as usize).max(1).min(5),
            exercise_ability: (normal.sample(rng).round() as usize).max(1).min(5),
            leadership_ability: (normal.sample(rng).round() as usize).max(1).min(5),
            needs_assistance: i < 3,
            gender: if i < 15 { Gender::Male } else { Gender::Female },
        })
        .collect::<Vec<Student>>();

    let mut seat_assignment = vec![vec![!0; 6]; 5];
    let mut student_ids = (0..30).collect::<Vec<usize>>();
    student_ids.shuffle(rng);
    for i in 0..30 {
        let (x, y) = (i % 6, i / 6);
        seat_assignment[y][x] = student_ids[i];
    }

    (seat_assignment, students)
}

pub fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

pub fn standard_deviation(values: &[f64]) -> f64 {
    let n = values.len() as f64;
    let mean = mean(values);
    let variance = values.iter().map(|&x| (x - mean).powf(2.0)).sum::<f64>() / n;
    variance.sqrt()
}

mod tests {
    use crate::structs::Gender;

    use super::*;

    #[test]
    fn test_swap_seats() {
        let mut seat_assignment = vec![vec![!0; 5]; 5];
        for j in 0..25 {
            let (x, y) = (j % 5, j / 5);
            seat_assignment[y][x] = j;
        }
        for i in 0..25 {
            let p1 = (i % 5, i / 5);
            for j in 0..25 {
                let p2 = (j % 5, j / 5);

                let (id1, id2) = (seat_assignment[p1.1][p1.0], seat_assignment[p2.1][p2.0]);
                swap_seats(&mut seat_assignment, p1, p2);

                assert_eq!(seat_assignment[p1.1][p1.0], id2);
                assert_eq!(seat_assignment[p2.1][p2.0], id1);
            }
        }
    }

    #[test]
    fn test_compress_student_id() {
        let mut students1 = (1..=15)
            .map(|i| Student {
                id: i,
                name: format!("Student {}", i),
                academic_ability: 3,
                exercise_ability: 3,
                leadership_ability: 3,
                needs_assistance: false,
                gender: Gender::Male,
            })
            .collect::<Vec<Student>>();
        let mut layout1 = vec![vec![!0; 4]; 4];
        for (i, student) in students1.iter().enumerate() {
            layout1[i / 4][i % 4] = student.id;
        }

        let students1_want = (0..15)
            .map(|i| Student {
                id: i,
                name: format!("Student {}", i + 1),
                academic_ability: 3,
                exercise_ability: 3,
                leadership_ability: 3,
                needs_assistance: false,
                gender: Gender::Male,
            })
            .collect::<Vec<Student>>();
        let mut layout1_want = vec![vec![!0; 4]; 4];
        for (i, student) in students1_want.iter().enumerate() {
            layout1_want[i / 4][i % 4] = student.id;
        }

        compress_student_id(&mut students1, &mut layout1);
        assert_eq!(students1, students1_want);
        assert_eq!(layout1, layout1_want);
    }
}
