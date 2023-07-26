use crate::{
    eval_func::evaluate,
    structs::{SeatAssignment, Student},
    utils::swap_seats,
};

use std::{
    collections::{BinaryHeap, VecDeque},
    io::Error,
};

pub fn beam_search(
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

                        let score = evaluate(previous, &new, students).unwrap();

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
        let score = evaluate(previous, &layout, students).unwrap();

        heap.push((score, layout));
    }

    if let Some((score, layout)) = heap.pop() {
        return Ok((layout, score));
    }

    Err(Error::new(std::io::ErrorKind::Other, "Beam search failed."))
}
