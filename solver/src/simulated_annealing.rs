use crate::{
    eval_func::evaluate,
    structs::{SeatAssignment, Student},
    utils::swap_seats,
};

use rand::Rng;
use rand_chacha::ChaCha20Rng;
use std::io::Error;

const LOOP_CNT: usize = 200000;
const T1: f64 = 119.5;
const T2: f64 = 1.563;

pub fn execute(
    previous: &SeatAssignment,
    students: &[Student],
    loop_cnt: usize,
    rng: &mut ChaCha20Rng,
    temperture1: f64,
    temperture2: f64,
) -> Result<(SeatAssignment, i64), Error> {
    let mut new = previous.clone();
    let mut best_score = evaluate(previous, &new, students).unwrap();

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

        if let Ok(new_score) = evaluate(previous, &new, students) {
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

mod tests {
    use rand::SeedableRng;
    use test::Bencher;

    use crate::{
        eval_func::individual,
        utils::{mean, standard_deviation, test_case},
    };

    use super::*;

    #[test]
    fn score_test_simulated_annealing() {
        let mut rng = ChaCha20Rng::seed_from_u64(123);

        let mut scores = vec![];
        let mut individual_scores = vec![];
        for _ in 0..100 {
            let (seat_assignment, students) = test_case(&mut rng);

            let res = execute(&seat_assignment, &students, LOOP_CNT, &mut rng, T1, T2);
            assert!(res.is_ok());
            let individual_score_sum =
                individual(&seat_assignment, &res.as_ref().unwrap().0, &students)
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

        b.iter(|| execute(&seat_assignment, &students, LOOP_CNT, &mut rng, T1, T2))
    }
}
