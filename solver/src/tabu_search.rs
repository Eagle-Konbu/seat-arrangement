use crate::{
    eval_func::evaluate,
    structs::{SeatAssignment, Student},
    utils::swap_seats,
};

use std::io::Error;

//TODO: 移動させて，離れた生徒同士のペアをタブーリストに入れる
pub fn execute<R: rand::Rng>(
    previous: &SeatAssignment,
    students: &[Student],
    loop_cnt: usize,
    neighbor_cnt: usize,
    tabu_list_size: usize,
    rng: &mut R,
) -> Result<(SeatAssignment, i64), Error> {

}
