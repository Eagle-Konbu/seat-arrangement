#![feature(test)]
extern crate test;

use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
    io::Error,
    vec,
};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crypto::{digest::Digest, sha2::Sha256};

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
        let mut hasher = Sha256::new();
        hasher.input_str(&seed_base_str);
        let hash_value = hasher.result_str();

        u64::from_str_radix(&hash_value[..8], 16).unwrap()
    };

    let mut rng = ChaCha20Rng::seed_from_u64(seed);

    simulated_annealing(previous, students, LOOP_CNT, &mut rng, T1, T2)
}

fn beam_search(
    previous: &SeatAssignment,
    students: &[Student],
    beam_width: usize,
) -> Result<(SeatAssignment, i64), Error> {
    let (depth, width, n) = (previous.len(), previous[0].len(), students.len());

    let mut deq = VecDeque::new();
    deq.push_back(previous.clone());

    for x1 in 0..width {
        for y1 in 0..depth {
            if previous[y1][x1] == !0 {
                continue;
            }

            let mut heap = BinaryHeap::new();
            loop {
                if deq.is_empty() {
                    break;
                }

                let current_layout = deq.pop_front().unwrap();
                for x2 in 0..width {
                    for y2 in 0..depth {
                        if current_layout[y2][x2] == !0 {
                            continue;
                        }

                        let mut new = current_layout.clone();
                        swap_seats(&mut new, (x1, y1), (x2, y2));

                        let score = eval_func(previous, &new, students).unwrap();

                        heap.push((score, new));
                    }
                }
            }

            for _ in 0..beam_width {
                if let Some((_, new)) = heap.pop() {
                    deq.push_back(new);
                }
            }
        }
    }

    let mut heap = BinaryHeap::new();
    loop {
        if deq.is_empty() {
            break;
        }

        let layout = deq.pop_front().unwrap();
        let score = eval_func(previous, &layout, students).unwrap();

        heap.push((score, layout));
    }

    if let Some((score, layout)) = heap.pop() {
        return Ok((layout, score));
    }

    Err(Error::new(std::io::ErrorKind::Other, "Beam search failed."))
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
    rng: &mut ChaCha20Rng,
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

const LOOP_CNT: usize = 20000;
const T1: f64 = 119.5;
const T2: f64 = 1.563;

const BEAM_WIDTH: usize = 10;

fn eval_func(
    previous: &SeatAssignment,
    new: &SeatAssignment,
    students: &[Student],
) -> Result<i64, Error> {
    let (depth, width, n) = (previous.len(), previous[0].len(), students.len());

    if let Ok(individual_scores) = individual_eval_func(previous, new, students) {
        let mut score = (individual_scores.iter().sum::<i64>() as f64 / n as f64) as i64;

        let (
            mut adj_academic_means,
            mut adj_exercise_means,
            mut adj_leadership_means,
            mut adj_male_rate,
        ) = (
            vec![0.0; students.len()],
            vec![0.0; students.len()],
            vec![0.0; students.len()],
            vec![0.0; students.len()],
        );

        for x in 0..width {
            for y in 0..depth {
                let student_id = new[y][x];
                if student_id == !0 {
                    continue;
                }

                let mut adj_academics = vec![students[student_id].academic_ability];
                let mut adj_exercises = vec![students[student_id].exercise_ability];
                let mut adj_leaderships = vec![students[student_id].leadership_ability];
                let mut adj_genders = vec![students[student_id].gender];
                for d in DIR {
                    if (x as i32 + d[0]) < 0
                        || (x as i32 + d[0]) >= new[0].len() as i32
                        || (y as i32 + d[1]) < 0
                        || (y as i32 + d[1]) >= new.len() as i32
                    {
                        continue;
                    }

                    let adj_student_id =
                        new[(y as i32 + d[1]) as usize][(x as i32 + d[0]) as usize];
                    if adj_student_id == !0 {
                        continue;
                    }

                    adj_academics.push(students[adj_student_id].academic_ability);
                    adj_exercises.push(students[adj_student_id].exercise_ability);
                    adj_leaderships.push(students[adj_student_id].leadership_ability);
                    adj_genders.push(students[adj_student_id].gender);
                }

                let male_cnt = adj_genders.iter().filter(|&&g| g == Gender::Male).count();

                adj_academic_means[student_id] =
                    adj_academics.iter().sum::<usize>() as f64 / adj_academics.len() as f64;
                adj_exercise_means[student_id] =
                    adj_exercises.iter().sum::<usize>() as f64 / adj_exercises.len() as f64;
                adj_leadership_means[student_id] =
                    adj_leaderships.iter().sum::<usize>() as f64 / adj_leaderships.len() as f64;
                adj_male_rate[student_id] = male_cnt as f64 / adj_genders.len() as f64;
            }
        }

        let (academic_min, academic_max) = (
            adj_academic_means
                .iter()
                .filter(|&&x| !x.is_nan())
                .min_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
            adj_academic_means
                .iter()
                .filter(|&&x| !x.is_nan())
                .max_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
        );
        let (exercise_min, exercise_max) = (
            adj_exercise_means
                .iter()
                .filter(|&&x| !x.is_nan())
                .min_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
            adj_exercise_means
                .iter()
                .filter(|&&x| !x.is_nan())
                .max_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
        );
        let (leadership_min, leadership_max) = (
            adj_leadership_means
                .iter()
                .filter(|&&x| !x.is_nan())
                .min_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
            adj_leadership_means
                .iter()
                .filter(|&&x| !x.is_nan())
                .max_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
        );

        let (male_rate_min, male_rate_max) = (
            adj_male_rate
                .iter()
                .filter(|&&x| !x.is_nan())
                .min_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
            adj_male_rate
                .iter()
                .filter(|&&x| !x.is_nan())
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;
    use rand_distr::{Distribution, Normal};

    use super::*;

    use test::Bencher;

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

    #[test]
    fn score_test_simulated_annealing() {
        let mut rng = ChaCha20Rng::seed_from_u64(123);

        let mut scores = vec![];
        let mut individual_scores = vec![];
        for _ in 0..100 {
            let (seat_assignment, students) = test_case(&mut rng);

            let res = simulated_annealing(&seat_assignment, &students, LOOP_CNT, &mut rng, T1, T2);
            assert!(res.is_ok());
            let individual_score_sum =
                individual_eval_func(&seat_assignment, &res.as_ref().unwrap().0, &students)
                    .unwrap()
                    .iter()
                    .sum::<i64>() as f64;
            scores.push(res.unwrap().1 as f64);
            individual_scores.push(individual_score_sum as f64 / students.len() as f64);
        }

        let (score_mean, score_sigma) = (mean(&scores), standard_deviation(&scores));

        println!("Mean: {}", score_mean);
        println!("Sigma: {}", score_sigma);
        println!("Mean(only individual): {}", mean(&individual_scores));
    }

    #[test]
    fn score_test_beam_search() {
        let mut scores = vec![];
        let mut individual_scores = vec![];

        let mut rng = ChaCha20Rng::seed_from_u64(123);
        for _ in 0..100 {
            let (seat_assignment, students) = test_case(&mut rng);

            let res = beam_search(&seat_assignment, &students, BEAM_WIDTH);
            assert!(res.is_ok());
            let individual_score_sum =
                individual_eval_func(&seat_assignment, &res.as_ref().unwrap().0, &students)
                    .unwrap()
                    .iter()
                    .sum::<i64>() as f64;
            scores.push(res.unwrap().1 as f64);
            individual_scores.push(individual_score_sum as f64 / students.len() as f64);
        }

        let (score_mean, score_sigma) = (mean(&scores), standard_deviation(&scores));

        println!("Mean: {}", score_mean);
        println!("Sigma: {}", score_sigma);
        println!("Mean(only individual): {}", mean(&individual_scores));
    }

    #[bench]
    fn bench_simulated_annealing(b: &mut Bencher) {
        let mut rng = ChaCha20Rng::seed_from_u64(123);

        let (seat_assignment, students) = test_case(&mut rng);

        b.iter(|| simulated_annealing(&seat_assignment, &students, LOOP_CNT, &mut rng, T1, T2))
    }

    #[bench]
    fn bench_beam_search(b: &mut Bencher) {
        let mut rng = ChaCha20Rng::seed_from_u64(123);

        let (seat_assignment, students) = test_case(&mut rng);

        b.iter(|| beam_search(&seat_assignment, &students, BEAM_WIDTH))
    }

    fn test_case(rng: &mut ChaCha20Rng) -> (SeatAssignment, Vec<Student>) {
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

    fn mean(values: &[f64]) -> f64 {
        values.iter().sum::<f64>() / values.len() as f64
    }

    fn standard_deviation(values: &[f64]) -> f64 {
        let n = values.len() as f64;
        let mean = mean(values);
        let variance = values.iter().map(|&x| (x - mean).powf(2.0)).sum::<f64>() / n;
        variance.sqrt()
    }
}
