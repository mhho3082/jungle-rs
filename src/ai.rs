use crate::{
    controller::{find_capture, list_all_moves},
    model::{State, DEN_BLUE, DEN_RED},
};

use rand::{rngs::ThreadRng, seq::SliceRandom};

pub fn _ai_random(state: &State, rng: &mut ThreadRng) -> (i32, i32) {
    *list_all_moves(state).choose(rng).unwrap()
}

pub fn _ai_naive(state: &State, rng: &mut ThreadRng) -> (i32, i32) {
    let all_moves = list_all_moves(state);

    // Get in den already!
    let win_moves: Vec<&(i32, i32)> = all_moves
        .iter()
        .filter(|(_, y)| {
            if state.cur_blue {
                y == &DEN_RED
            } else {
                y == &DEN_BLUE
            }
        })
        .collect();
    if !win_moves.is_empty() {
        return **win_moves.choose(rng).unwrap();
    } else {
        // Capture if possible
        let attack_moves: Vec<&(i32, i32)> = all_moves
            .iter()
            .filter(|(_, y)| find_capture(state, *y))
            .collect();
        if !attack_moves.is_empty() {
            return **attack_moves.choose(rng).unwrap();
        } else {
            // Just randomly pick one
            return *all_moves.choose(rng).unwrap();
        }
    }
}
