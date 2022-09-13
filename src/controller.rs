// My thought is that these would mainly be
// modifications on the model

// Move: (index, moveTo)

// TODO:
// check_move
// check_win
// make_move
// list_all_moves
// list_piece_moves
//
// possibly also handle history:
// state_count
// goto_state

use crate::model::{State, DENS};

pub fn check_win(state: &State) -> bool {
    state.board.blue.contains(&DENS[0])
        || state.board.red.contains(&DENS[1])
        || state.board.blue.iter().all(|x| x > &62)
        || state.board.red.iter().all(|x| x > &62)
}
