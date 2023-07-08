use std::io::Error;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[warn(overflowing_literals)]
fn eval_func(
    previous: &SeatAssignment,
    new: &SeatAssignment,
    students: Vec<Student>,
) -> Result<i64, Error> {
    let (height, width, n) = (previous.len(), previous[0].len(), students.len());

    if let Ok(individual_scores) = individual_eval_func(previous, new, students) {
        let score = individual_scores.iter().sum();
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
    students: Vec<Student>,
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

    let mut prev_adj_distance_averages = vec![0.0; n];
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
        prev_adj_distance_averages[i] = sum / prev_adj_student_ids.len() as f64;
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
        individual_scores[i] = (prev_adj_distance_averages[i] * 10000.0) as i64;
        if students[i].needs_assistance {
            let distance_penalty = blackboard_distances[i].exp() as i64;
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
#[derive(Clone, Debug, PartialEq, Eq)]
struct Student {
    id: usize,
    name: String,
    academic_ability: usize,
    exercise_ability: usize,
    leadership: usize,
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
