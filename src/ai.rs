use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{controller::list_all_moves, model::State};

pub fn ai_random(state: &State, rng: &mut ThreadRng) -> (i32, i32) {
    *list_all_moves(state).choose(rng).unwrap()
}
