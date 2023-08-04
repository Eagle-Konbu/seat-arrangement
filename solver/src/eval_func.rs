use std::io::Error;

use crate::structs::{Gender, SeatAssignment, Student};

const PREV_ADJ_DISTANCE_WEIGHT: f64 = 1000.0;
const BLACKBOARD_DISTANCE_WEIGHT: f64 = 1000.0;
const ACADEMIC_WEIGHT: f64 = 1000.0;
const EXERCISE_WEIGHT: f64 = 1000.0;
const LEADERSHIP_WEIGHT: f64 = 1000.0;
const GENDER_WEIGHT: f64 = 1000.0;

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

pub fn evaluate(
    previous: &SeatAssignment,
    new: &SeatAssignment,
    students: &[Student],
) -> Result<i64, Error> {
    let (depth, width, n) = (previous.len(), previous[0].len(), students.len());

    if let Ok(individual_scores) = individual(previous, new, students) {
        let mut score = (individual_scores.iter().sum::<i64>() as f64 / n as f64) as i64;

        let (
            mut adj_academic_means,
            mut adj_exercise_means,
            mut adj_leadership_means,
            mut adj_male_rate,
            mut adj_cnt,
        ) = (
            vec![vec![0.0; width]; depth],
            vec![vec![0.0; width]; depth],
            vec![vec![0.0; width]; depth],
            vec![vec![0.0; width]; depth],
            vec![vec![0; width]; depth],
        );

        for x in 0..width {
            for y in 0..depth {
                let student_id = new[y][x];
                if student_id == !0 {
                    continue;
                }

                let imos_pos_scaler = [
                    (x as i64 - 1, y as i64 - 1, 1),
                    (x as i64 + 2, y as i64 - 1, -1),
                    (x as i64 - 1, y as i64 + 2, -1),
                    (x as i64 + 2, y as i64 + 2, 1),
                ];

                for &(x2, y2, scaler) in imos_pos_scaler.iter() {
                    let (i, j) = (y2.max(0) as usize, x2.max(0) as usize);

                    if i >= depth || j >= width {
                        continue;
                    }

                    adj_academic_means[i][j] +=
                        (scaler * students[student_id].academic_ability as i64) as f64;
                    adj_exercise_means[i][j] +=
                        (scaler * students[student_id].exercise_ability as i64) as f64;
                    adj_leadership_means[i][j] +=
                        (scaler * students[student_id].leadership_ability as i64) as f64;
                    adj_male_rate[i][j] += (scaler
                        * if students[student_id].gender == Gender::Male {
                            1
                        } else {
                            0
                        }) as f64;
                    adj_cnt[i][j] += scaler;
                }
            }
        }

        for y in 0..depth {
            for x in 1..width {
                adj_academic_means[y][x] += adj_academic_means[y][x - 1];
                adj_exercise_means[y][x] += adj_exercise_means[y][x - 1];
                adj_leadership_means[y][x] += adj_leadership_means[y][x - 1];
                adj_male_rate[y][x] += adj_male_rate[y][x - 1];
                adj_cnt[y][x] += adj_cnt[y][x - 1];
            }
        }

        for x in 0..width {
            for y in 1..depth {
                adj_academic_means[y][x] += adj_academic_means[y - 1][x];
                adj_exercise_means[y][x] += adj_exercise_means[y - 1][x];
                adj_leadership_means[y][x] += adj_leadership_means[y - 1][x];
                adj_male_rate[y][x] += adj_male_rate[y - 1][x];
                adj_cnt[y][x] += adj_cnt[y - 1][x];
            }
        }

        for x in 0..width {
            for y in 0..depth {
                adj_academic_means[y][x] /= adj_cnt[y][x] as f64;
                adj_exercise_means[y][x] /= adj_cnt[y][x] as f64;
                adj_leadership_means[y][x] /= adj_cnt[y][x] as f64;
                adj_male_rate[y][x] /= adj_cnt[y][x] as f64;
            }
        }

        let (academic_min, academic_max) = (
            adj_academic_means
                .iter()
                .flatten()
                .filter(|&&x| !x.is_nan())
                .min_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
            adj_academic_means
                .iter()
                .flatten()
                .filter(|&&x| !x.is_nan())
                .max_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
        );
        let (exercise_min, exercise_max) = (
            adj_exercise_means
                .iter()
                .flatten()
                .filter(|&&x| !x.is_nan())
                .min_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
            adj_exercise_means
                .iter()
                .flatten()
                .filter(|&&x| !x.is_nan())
                .max_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
        );
        let (leadership_min, leadership_max) = (
            adj_leadership_means
                .iter()
                .flatten()
                .filter(|&&x| !x.is_nan())
                .min_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
            adj_leadership_means
                .iter()
                .flatten()
                .filter(|&&x| !x.is_nan())
                .max_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
        );

        let (male_rate_min, male_rate_max) = (
            adj_male_rate
                .iter()
                .flatten()
                .filter(|&&x| !x.is_nan())
                .min_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap(),
            adj_male_rate
                .iter()
                .flatten()
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

pub fn individual(
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
        let mut sum = 0.0;
        let (x_prev, y_prev) = before_after_positions[i].0;
        let mut prev_adj_cnt = 0;
        for d in DIR {
            let (x, y) = ((x_prev as i32 + d[0]), (y_prev as i32 + d[1]));
            if x < 0 || x >= width as i32 || y < 0 || y >= depth as i32 {
                continue;
            }
            let adj_student_id = previous[y as usize][x as usize];
            if adj_student_id != !0 {
                let (x1, y1) = before_after_positions[i].1;
                let (x2, y2) = before_after_positions[adj_student_id].1;

                sum += ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as f64;
                prev_adj_cnt += 1;
            }
        }

        prev_adj_distance_means[i] = sum / prev_adj_cnt as f64;
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
