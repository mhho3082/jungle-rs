use crate::{
    controller::{find_capture, list_all_moves},
    model::{State, COL_COUNT, DEN_BLUE, DEN_RED},
};

use rand::{
    distributions::WeightedIndex, prelude::Distribution, rngs::ThreadRng,
    seq::SliceRandom,
};

pub fn _ai_random(state: &State, rng: &mut ThreadRng) -> (i32, i32) {
    *list_all_moves(state).choose(rng).unwrap()
}

/// Randomly picks move if nothing to do
/// This means that the AI doesn't really try to move towards enemy's den
pub fn _ai_naive_defensive(state: &State, rng: &mut ThreadRng) -> (i32, i32) {
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
        **win_moves.choose(rng).unwrap()
    } else {
        // Capture if possible
        let attack_moves: Vec<&(i32, i32)> = all_moves
            .iter()
            .filter(|(_, y)| find_capture(state, *y))
            .collect();
        if !attack_moves.is_empty() {
            **attack_moves.choose(rng).unwrap()
        } else {
            // Just randomly pick one
            *all_moves.choose(rng).unwrap()
        }
    }
}

/// Always pick from the farthest moves if nothing to do
pub fn _ai_naive_aggressive(state: &State, rng: &mut ThreadRng) -> (i32, i32) {
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
        **win_moves.choose(rng).unwrap()
    } else {
        // Capture if possible
        let attack_moves: Vec<&(i32, i32)> = all_moves
            .iter()
            .filter(|(_, y)| find_capture(state, *y))
            .collect();
        if !attack_moves.is_empty() {
            **attack_moves.choose(rng).unwrap()
        } else if state.cur_blue {
            // Find farthest move (blue)
            let mut farthest: i32 = 10;
            for (_, y) in &all_moves {
                if (y / COL_COUNT) < farthest {
                    farthest = y / COL_COUNT;
                }
            }
            **all_moves
                .iter()
                .filter(|(_, y)| (y / COL_COUNT) == farthest)
                .collect::<Vec<&(i32, i32)>>()
                .choose(rng)
                .unwrap()
        } else {
            // Find farthest move (red)
            let mut farthest: i32 = 0;
            for (_, y) in &all_moves {
                if (y / COL_COUNT) > farthest {
                    farthest = y / COL_COUNT;
                }
            }
            **all_moves
                .iter()
                .filter(|(_, y)| (y / COL_COUNT) == farthest)
                .collect::<Vec<&(i32, i32)>>()
                .choose(rng)
                .unwrap()
        }
    }
}

/// Picks move randomly (distributed with farness) when nothing to do
pub fn _ai_naive_neutral(state: &State, rng: &mut ThreadRng) -> (i32, i32) {
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
        **win_moves.choose(rng).unwrap()
    } else {
        // Capture if possible
        let attack_moves: Vec<&(i32, i32)> = all_moves
            .iter()
            .filter(|(_, y)| find_capture(state, *y))
            .collect();
        if !attack_moves.is_empty() {
            **attack_moves.choose(rng).unwrap()
        } else if state.cur_blue {
            // Generate distribution
            let dist_base = all_moves
                .iter()
                .map(|(_, y)| (10 - (y / COL_COUNT)))
                .collect::<Vec<i32>>();
            let dist = WeightedIndex::new(&dist_base).unwrap();

            // Randomize move
            all_moves[dist.sample(rng)]
        } else {
            // Generate distribution
            let dist_base = all_moves
                .iter()
                .map(|(_, y)| y / COL_COUNT)
                .collect::<Vec<i32>>();
            let dist = WeightedIndex::new(&dist_base).unwrap();

            // Randomize move
            all_moves[dist.sample(rng)]
        }
    }
}
