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

/// Assumes that the move is legal already
pub fn make_move(state: &State, piece: i32, move_to: i32) -> State {
    let mut new_state = *state;

    // Move piece
    if new_state.cur_blue {
        new_state.board.blue[piece as usize] = move_to;
    } else {
        new_state.board.red[piece as usize] = move_to;
    }

    // Toggles switches
    new_state.cur_blue = !new_state.cur_blue;
    new_state.won = check_win(&new_state);

    new_state
}

pub fn check_win(state: &State) -> bool {
    state.board.blue.contains(&DENS[0])
        || state.board.red.contains(&DENS[1])
        || state.board.blue.iter().all(|x| x > &62)
        || state.board.red.iter().all(|x| x > &62)
}
