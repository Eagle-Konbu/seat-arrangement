use crate::{
    eval_func::evaluate,
    structs::{SeatAssignment, Student},
    utils::swap_seats,
};

use std::{collections::{BTreeSet, VecDeque}, io::Error};

pub fn execute<R: rand::Rng>(
    previous: &SeatAssignment,
    students: &[Student],
    loop_cnt: usize,
    neighbor_cnt: usize,
    tabu_list_size: usize,
    rng: &mut R,
) -> Result<(SeatAssignment, i64), Error> {
    let mut tabu_list = BTreeSet::new();
    let mut deq = VecDeque::new();

    let mut current = previous.clone();

    let mut best = evaluate(previous, &current, students).unwrap();
    
    for _ in 0..loop_cnt {
        for _ in 0..neighbor_cnt {
            let (pos1, pos2) = loop {
                let pos1 = (rng.gen_range(0..previous.len()), rng.gen_range(0..previous[0].len()));
                let pos2 = (rng.gen_range(0..previous.len()), rng.gen_range(0..previous[0].len()));
                if pos1 != pos2 && !tabu_list.contains(&(students[previous[pos1.0][pos1.1]].id, students[previous[pos2.0][pos2.1]].id)) {
                    break (pos1, pos2);
                }
            };

            swap_seats(&mut current, pos1, pos2);
            let score = evaluate(previous, &current, students).unwrap();
            if score < best {
                best = score;
            } else {
                swap_seats(&mut current, pos1, pos2);
                continue;
            }

            tabu_list.insert((students[previous[pos1.0][pos1.1]].id, students[previous[pos2.0][pos2.1]].id));
            deq.push_back((students[previous[pos1.0][pos1.1]].id, students[previous[pos2.0][pos2.1]].id));

            while deq.len() > tabu_list_size {
                let (id1, id2) = deq.pop_front().unwrap();
                tabu_list.remove(&(id1, id2));
            }
        }
    }

    Ok((current, best))
}
