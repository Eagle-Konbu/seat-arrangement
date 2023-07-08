use std::io::Error;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

const PREV_ADJ_DISTANCE_WEIGHT: f64 = 10000.0;
const ACCADEMIC_WEIGHT: f64 = 1000.0;
const EXERCISE_WEIGHT: f64 = 1000.0;
const LEADERSHIP_WEIGHT: f64 = 1000.0;
const GENDER_WEIGHT: f64 = 1000.0;

#[warn(overflowing_literals)]
fn eval_func(
    previous: &SeatAssignment,
    new: &SeatAssignment,
    students: &[Student],
) -> Result<i64, Error> {
    let (depth, width, n) = (previous.len(), previous[0].len(), students.len());

    if let Ok(individual_scores) = individual_eval_func(previous, new, students) {
        let mut score = individual_scores.iter().sum();

        let (
            mut adj_accademic_means,
            mut adj_exercise_means,
            mut adj_leadership_means,
            mut adj_gender_means,
        ) = (vec![0.0; n], vec![0.0; n], vec![0.0; n], vec![0.0; n]);

        for y1 in 0..depth {
            for x1 in 0..width {
                let student_id = new[y1][x1];
                if student_id == !0 {
                    continue;
                }

                let mut student_ids_to_be_counted = vec![student_id];
                for d in DIR {
                    let (x2, y2) = ((x1 as i32 + d[0]), (y1 as i32 + d[1]));
                    if x2 < 0 || x2 >= width as i32 || y2 < 0 || y2 >= depth as i32 {
                        continue;
                    }
                    let adj_student_id = new[y2 as usize][x2 as usize];
                    if adj_student_id != !0 {
                        student_ids_to_be_counted.push(adj_student_id);
                    }
                }

                adj_accademic_means[student_id] = student_ids_to_be_counted
                    .iter()
                    .map(|&id| students[id].academic_ability)
                    .sum::<usize>() as f64
                    / student_ids_to_be_counted.len() as f64;
                adj_exercise_means[student_id] = student_ids_to_be_counted
                    .iter()
                    .map(|&id| students[id].exercise_ability)
                    .sum::<usize>() as f64
                    / student_ids_to_be_counted.len() as f64;
                adj_leadership_means[student_id] = student_ids_to_be_counted
                    .iter()
                    .map(|&id| students[id].leadership_ability)
                    .sum::<usize>() as f64
                    / student_ids_to_be_counted.len() as f64;
                adj_gender_means[student_id] = student_ids_to_be_counted
                    .iter()
                    .map(|&id| {
                        if students[id].gender == Gender::Female {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<usize>() as f64
                    / student_ids_to_be_counted.len() as f64;
            }
        }

        let (accademic_deviationn, exercise_deviation, leadership_deviation, gender_deviation) = (
            standard_deviaton(&adj_accademic_means),
            standard_deviaton(&adj_exercise_means),
            standard_deviaton(&adj_leadership_means),
            standard_deviaton(&adj_gender_means),
        );

        score -= (accademic_deviationn * ACCADEMIC_WEIGHT) as i64;
        score -= (exercise_deviation * EXERCISE_WEIGHT) as i64;
        score -= (leadership_deviation * LEADERSHIP_WEIGHT) as i64;
        score -= (gender_deviation * GENDER_WEIGHT) as i64;

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
        let (x1, y1) = before_after_positions[i].0;
        let mut prev_adj_student_ids = vec![];
        for d in DIR {
            let (x, y) = ((x1 as i32 + d[0]), (y1 as i32 + d[1]));
            if x < 0 || x >= width as i32 || y < 0 || y >= depth as i32 {
                continue;
            }
            let adj_student_id = previous[y as usize][x as usize];
            if adj_student_id != !0 {
                prev_adj_student_ids.push(adj_student_id);
            }
        }

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
            let distance_penalty = blackboard_distances[i].exp() as i64;
            individual_scores[i] -= distance_penalty;
        }
    }

    Ok(individual_scores)
}

fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

fn standard_deviaton(values: &[f64]) -> f64 {
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
#[derive(Clone, Debug, PartialEq, Eq)]
struct Student {
    id: usize,
    name: String,
    academic_ability: usize,
    exercise_ability: usize,
    leadership_ability: usize,
    needs_assistance: bool,
    gender: Gender,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Gender {
    Male,
    Female,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
