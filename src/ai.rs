use crate::{
    controller::{find_capture, list_all_moves},
    model::{
        State, COL_COUNT, DEN_BLUE, DEN_RED, ROW_COUNT, TRAPS_BLUE, TRAPS_RED,
    },
};

use rand::{
    distributions::WeightedIndex, prelude::Distribution, rngs::ThreadRng,
    seq::SliceRandom,
};

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum AIType {
    Random,
    NaiveDefensive,
    NaiveAggressive,
    None,
}

impl AIType {
    /// Decide a move depending on the AI type
    pub fn decide_move(
        &self,
        state: &State,
        rng: &mut ThreadRng,
    ) -> (i32, i32) {
        match &self {
            AIType::Random | AIType::None => ai_random(state, rng),
            AIType::NaiveDefensive => ai_naive_defensive(state, rng),
            AIType::NaiveAggressive => ai_naive_aggressive(state, rng),
        }
    }
}

/// Randomly pick a move
pub fn ai_random(state: &State, rng: &mut ThreadRng) -> (i32, i32) {
    *list_all_moves(state).choose(rng).unwrap()
}

/// Can avoid attack if possible
pub fn ai_naive_defensive(state: &State, rng: &mut ThreadRng) -> (i32, i32) {
    let all_moves = list_all_moves(state);

    if let Some(action) = pick_win(state, &all_moves, rng) {
        // Win if possible
        action
    } else if let Some(action) = pick_attack(state, &all_moves, rng) {
        // Attack if possible
        action
    } else if let Some(action) = pick_avoid_attack(state, &all_moves, rng) {
        // Avoid attacks if possible
        action
    } else {
        // Randomly pick one based on distribution
        pick_distribution_farthest(state, &all_moves, rng)
    }
}

/// Doesn't avoid attacks, and will dash for the farthest move
pub fn ai_naive_aggressive(state: &State, rng: &mut ThreadRng) -> (i32, i32) {
    let all_moves = list_all_moves(state);

    if let Some(action) = pick_win(state, &all_moves, rng) {
        // Win if possible
        action
    } else if let Some(action) = pick_attack(state, &all_moves, rng) {
        // Attack if possible
        action
    } else {
        // Pick from a farthest move
        pick_farthest(state, &all_moves, rng)
    }
}

fn pick_win(
    state: &State,
    all_moves: &[(i32, i32)],
    rng: &mut ThreadRng,
) -> Option<(i32, i32)> {
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
    win_moves.choose(rng).map(|id| **id)
}

fn pick_attack(
    state: &State,
    all_moves: &[(i32, i32)],
    rng: &mut ThreadRng,
) -> Option<(i32, i32)> {
    let attack_moves: Vec<&(i32, i32)> = all_moves
        .iter()
        .filter(|(_, y)| find_capture(state, *y))
        .collect();
    attack_moves.choose(rng).map(|id| **id)
}

fn pick_avoid_attack(
    state: &State,
    all_moves: &[(i32, i32)],
    rng: &mut ThreadRng,
) -> Option<(i32, i32)> {
    // Create the world from opponent's eyes
    let mut opposite_state = *state;
    opposite_state.cur_blue ^= true;
    let opposite_moves = list_all_moves(&opposite_state);

    // Avoid not-move-then-attacked
    let attacked_pieces: Vec<i32> = (0..8)
        .filter(|&piece| {
            let original = if state.cur_blue {
                state.board.blue[piece as usize]
            } else {
                state.board.red[piece as usize]
            };

            matches!(
                opposite_moves.iter().position(|(_, y)| y == &original),
                Some(_)
            )
        })
        .collect();

    let avoid_moves: Vec<&(i32, i32)> = all_moves
        .iter()
        .filter(|&(piece, _)| attacked_pieces.contains(piece))
        .collect();

    if !avoid_moves.is_empty() {
        return avoid_moves.choose(rng).map(|id| **id);
    }

    // Avoid move-then-attacked
    let good_moves: Vec<&(i32, i32)> = all_moves
        .iter()
        .filter(|&(piece, move_to)| {
            let mut safe = true;
            for (enemy, their_move_to) in &opposite_moves {
                if move_to == their_move_to {
                    if enemy >= piece || (*enemy == 0 && *piece == 7) {
                        safe = false;
                        break;
                    } else if state.cur_blue {
                        if TRAPS_BLUE.contains(move_to) {
                            safe = false;
                            break;
                        }
                    } else if TRAPS_RED.contains(move_to) {
                        safe = false;
                        break;
                    }
                }
            }
            safe
        })
        .collect();
    good_moves.choose(rng).map(|id| **id)
}

fn pick_farthest(
    state: &State,
    all_moves: &[(i32, i32)],
    rng: &mut ThreadRng,
) -> (i32, i32) {
    if state.cur_blue {
        // Find farthest move (blue)
        let mut farthest: i32 = ROW_COUNT;
        for (_, y) in all_moves {
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
        for (_, y) in all_moves {
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

fn pick_distribution_farthest(
    state: &State,
    all_moves: &[(i32, i32)],
    rng: &mut ThreadRng,
) -> (i32, i32) {
    if state.cur_blue {
        // Generate distribution
        let dist_base = all_moves
            .iter()
            .map(|(_, y)| (ROW_COUNT - (y / COL_COUNT)))
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
