#![feature(test)]
extern crate test;

use std::{collections::HashSet, io::Error, vec};

use rand::{rngs::ThreadRng, Rng};

fn solve(previous: &SeatAssignment, students: &[Student]) -> Result<(SeatAssignment, i64), Error> {
    let mut rng = rand::thread_rng();

    simulated_annealing(previous, students, 10000, &mut rng, 500.0, 0.0)
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

fn separate_input(input: &[Vec<Option<Student>>]) -> (SeatAssignment, Vec<Student>) {
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

    for i in 0..students.len() {
        students[i].id = i;
    }

    (idx_seat_assignment, students)
}

fn check_input(input: &[Vec<Option<Student>>]) -> Result<(), Error> {
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

fn compress_student_id(students: &mut [Student], idx_layout: &mut SeatAssignment) {
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

fn simulated_annealing(
    previous: &SeatAssignment,
    students: &[Student],
    loop_cnt: usize,
    rng: &mut ThreadRng,
    temperture1: f64,
    temperture2: f64,
) -> Result<(SeatAssignment, i64), Error> {
    let mut new = previous.clone();
    let mut best_score = eval_func(previous, &new, students).unwrap();

    let (depth, width) = (previous.len(), previous[0].len());

    for i in 0..loop_cnt {
        let (pos1, pos2) = (
            (rng.gen_range(0..width), rng.gen_range(0..depth)),
            (rng.gen_range(0..width), rng.gen_range(0..depth)),
        );

        if previous[pos1.1][pos1.0] == !0 || previous[pos2.1][pos2.0] == !0 {
            continue;
        }

        let temperture = temperture1 + (temperture2 - temperture1) * i as f64 / loop_cnt as f64;

        swap_seats(&mut new, pos1, pos2);

        if let Ok(new_score) = eval_func(previous, &new, students) {
            let p = ((new_score - best_score) as f64 / temperture).exp();
            if new_score > best_score || rng.gen_bool(p) {
                best_score = new_score;
            } else {
                swap_seats(&mut new, pos1, pos2);
            }
        } else {
            swap_seats(&mut new, pos1, pos2);
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "eval_func() returned an error",
            ));
        }
    }

    Ok((new, best_score))
}

fn swap_seats(assigment: &mut SeatAssignment, pos1: (usize, usize), pos2: (usize, usize)) {
    let tmp = assigment[pos1.1][pos1.0];
    assigment[pos1.1][pos1.0] = assigment[pos2.1][pos2.0];
    assigment[pos2.1][pos2.0] = tmp;
}

const PREV_ADJ_DISTANCE_WEIGHT: f64 = 1000.0;
const BLACKBOARD_DISTANCE_WEIGHT: f64 = 1000.0;
const ACADEMIC_WEIGHT: f64 = 1000.0;
const EXERCISE_WEIGHT: f64 = 1000.0;
const LEADERSHIP_WEIGHT: f64 = 1000.0;
const GENDER_WEIGHT: f64 = 1000.0;
const GROUP_SIZE: usize = 3;

#[warn(overflowing_literals)]
fn eval_func(
    previous: &SeatAssignment,
    new: &SeatAssignment,
    students: &[Student],
) -> Result<i64, Error> {
    let (depth, width, n) = (previous.len(), previous[0].len(), students.len());

    if let Ok(individual_scores) = individual_eval_func(previous, new, students) {
        let mut score = (individual_scores.iter().sum::<i64>() as f64 / n as f64) as i64;

        let group = (0..depth)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        x / GROUP_SIZE
                            + (y / GROUP_SIZE) * (width as f64 / GROUP_SIZE as f64).ceil() as usize
                    })
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        let group_cnt = (width as f64 / GROUP_SIZE as f64).ceil() as usize
            * (depth as f64 / GROUP_SIZE as f64).ceil() as usize;

        let (
            mut adj_academic_means,
            mut adj_exercise_means,
            mut adj_leadership_means,
            mut adj_male_rate,
        ) = (
            vec![0.0; group_cnt],
            vec![0.0; group_cnt],
            vec![0.0; group_cnt],
            vec![0.0; group_cnt],
        );

        let mut group_member_cnts = vec![0; group_cnt];
        let mut male_female_rate = 0.0;
        for x in 0..width {
            for y in 0..depth {
                if new[y][x] == !0 {
                    continue;
                }

                adj_academic_means[group[y][x]] += students[new[y][x]].academic_ability as f64;
                adj_exercise_means[group[y][x]] += students[new[y][x]].exercise_ability as f64;
                adj_leadership_means[group[y][x]] += students[new[y][x]].leadership_ability as f64;
                adj_male_rate[group[y][x]] += if students[new[y][x]].gender == Gender::Male {
                    1.0
                } else {
                    0.0
                };

                group_member_cnts[group[y][x]] += 1;
            }
        }
        for i in 0..group_cnt {
            adj_academic_means[i] /= group_member_cnts[i] as f64;
            adj_exercise_means[i] /= group_member_cnts[i] as f64;
            adj_leadership_means[i] /= group_member_cnts[i] as f64;
            adj_male_rate[i] /= group_member_cnts[i] as f64;
        }

        let (academic_min, academic_max) = (
            adj_academic_means
                .iter()
                .min_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
            adj_academic_means
                .iter()
                .max_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
        );
        let (exercise_min, exercise_max) = (
            adj_exercise_means
                .iter()
                .min_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
            adj_exercise_means
                .iter()
                .max_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
        );
        let (leadership_min, leadership_max) = (
            adj_leadership_means
                .iter()
                .min_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
            adj_leadership_means
                .iter()
                .max_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
        );

        let (male_rate_min, male_rate_max) = (
            adj_male_rate
                .iter()
                .min_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
            adj_male_rate
                .iter()
                .max_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
        );

        score += (ACADEMIC_WEIGHT * (academic_min / academic_max)) as i64;
        score += (EXERCISE_WEIGHT * (exercise_min / exercise_max)) as i64;
        score += (LEADERSHIP_WEIGHT * (leadership_min / leadership_max)) as i64;
        score += (GENDER_WEIGHT * (male_rate_min / male_rate_max)) as i64;

        return Ok(score);
    }

    Err(Error::new(
        std::io::ErrorKind::Other,
        "Something went wrong",
    ))
}

#[warn(overflowing_literals)]
fn individual_eval_func(
    previous: &SeatAssignment,
    new: &SeatAssignment,
    students: &[Student],
) -> Result<Vec<i64>, Error> {
    let (depth, width, n) = (previous.len(), previous[0].len(), students.len());

    // distance between prev_adj_students and student
    let mut before_after_positions = vec![((!0, !0), (!0, !0)); n];
    for y in 0..depth {
        for x in 0..width {
            let before_student_id = previous[y][x];
            let after_student_id = new[y][x];

            if (before_student_id == !0 && after_student_id != !0)
                || (before_student_id != !0 && after_student_id == !0)
            {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid input",
                ));
            }

            if before_student_id == !0 {
                continue;
            }

            before_after_positions[before_student_id].0 = (x, y);
            before_after_positions[after_student_id].1 = (x, y);
        }
    }

    let mut prev_adj_distance_means = vec![0.0; n];
    for i in 0..n {
        let (x_prev, y_prev) = before_after_positions[i].0;
        let mut prev_adj_student_ids = vec![];
        for d in DIR {
            let (x, y) = ((x_prev as i32 + d[0]), (y_prev as i32 + d[1]));
            if x < 0 || x >= width as i32 || y < 0 || y >= depth as i32 {
                continue;
            }
            let adj_student_id = previous[y as usize][x as usize];
            if adj_student_id != !0 {
                prev_adj_student_ids.push(adj_student_id);
            }
        }

        let (x1, y1) = before_after_positions[i].1;

        let mut sum = 0.0;
        for j in 0..prev_adj_student_ids.len() {
            let (x2, y2) = before_after_positions[prev_adj_student_ids[j]].0;
            sum += ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as f64;
        }
        prev_adj_distance_means[i] = sum / prev_adj_student_ids.len() as f64;
    }

    // distance between blackboard and student
    let mut blackboard_distances = vec![0.0; n];
    let (x_blackboard, y_blackboard) = (width as f64 / 2.0, -1.0);
    for i in 0..n {
        let (x, y) = before_after_positions[i].1;
        blackboard_distances[i] =
            ((x as f64 - x_blackboard).powf(2.0) + (y as f64 - y_blackboard).powf(2.0)).sqrt();
    }

    let mut individual_scores = vec![0; n];
    for i in 0..n {
        individual_scores[i] = (prev_adj_distance_means[i] * PREV_ADJ_DISTANCE_WEIGHT) as i64;
        if students[i].needs_assistance {
            let distance_penalty = (blackboard_distances[i] * BLACKBOARD_DISTANCE_WEIGHT) as i64;
            individual_scores[i] -= distance_penalty;
        }
    }

    Ok(individual_scores)
}

fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

fn standard_deviation(values: &[f64]) -> f64 {
    let n = values.len() as f64;
    let mean = mean(values);
    let variance = values.iter().map(|&x| (x - mean).powf(2.0)).sum::<f64>() / n;
    variance.sqrt()
}

type SeatAssignment = Vec<Vec<usize>>;
const DIR: [[i32; 2]; 8] = [
    [0, 1],
    [1, 0],
    [0, -1],
    [-1, 0],
    [-1, -1],
    [1, 1],
    [-1, 1],
    [1, -1],
];

// id must be unique and 0-indexed
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Student {
    pub id: usize,
    name: String,
    academic_ability: usize,
    exercise_ability: usize,
    leadership_ability: usize,
    needs_assistance: bool,
    gender: Gender,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;

    use super::*;

    use test::Bencher;

    #[test]
    fn test_sa() {
        let mut rng = rand::thread_rng();

        let (mut score_mean, mut score_sigma) = (0.0, 0.0);

        let mut scores = vec![];
        let mut individual_scores = vec![];
        for _ in 0..100 {
            let students = (0..30)
                .map(|i| Student {
                    id: i,
                    name: format!("Student {}", i),
                    academic_ability: rng.gen_range(1..=5),
                    exercise_ability: rng.gen_range(1..=5),
                    leadership_ability: rng.gen_range(1..=5),
                    needs_assistance: i < 3,
                    gender: if i < 15 { Gender::Male } else { Gender::Female },
                })
                .collect::<Vec<Student>>();

            let mut seat_assignment = vec![vec![!0; 6]; 5];
            let mut student_ids = (0..30).collect::<Vec<usize>>();
            student_ids.shuffle(&mut rng);
            for i in 0..30 {
                let (x, y) = (i % 6, i / 6);
                seat_assignment[y][x] = student_ids[i];
            }

            let res = solve(&seat_assignment, &students);
            assert!(res.is_ok());
            let individual_score_sum =
                individual_eval_func(&seat_assignment, &res.as_ref().unwrap().0, &students)
                    .unwrap()
                    .iter()
                    .sum::<i64>() as f64;
            scores.push(res.unwrap().1 as f64);
            individual_scores.push(individual_score_sum as f64 / students.len() as f64);
        }

        score_mean = mean(&scores);
        score_sigma = standard_deviation(&scores);

        println!("Mean: {}", score_mean);
        println!("Sigma: {}", score_sigma);
        println!("Mean(only individual): {}", mean(&individual_scores));
    }

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let students = (0..30)
            .map(|i| Student {
                id: i,
                name: format!("Student {}", i),
                academic_ability: rng.gen_range(1..=5),
                exercise_ability: rng.gen_range(1..=5),
                leadership_ability: rng.gen_range(1..=5),
                needs_assistance: rng.gen_bool(0.1),
                gender: if rng.gen_bool(0.5) {
                    Gender::Male
                } else {
                    Gender::Female
                },
            })
            .collect::<Vec<Student>>();

        let mut seat_assignment = vec![vec![!0; 6]; 5];
        let mut student_ids = (0..30).collect::<Vec<usize>>();
        student_ids.shuffle(&mut rng);
        for i in 0..30 {
            let (x, y) = (i % 6, i / 6);
            seat_assignment[y][x] = student_ids[i];
        }

        b.iter(|| solve(&seat_assignment, &students))
    }
}
