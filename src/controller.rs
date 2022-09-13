// My thought is that these would mainly be
// modifications on the model

// Move: (index, moveTo)

// TODO:
// check_move
// check_win
// make_move
//
// For UI and AI:
// list_moves
// list_piece_moves
//
// possibly also handle history:
// state_count
// goto_state

use crate::model::{State, COL_COUNT, DEN_BLUE, DEN_RED, RIVERS, TRAPS_BLUE, TRAPS_RED};

pub fn check_move(state: &State, piece: i32, move_to: i32) -> bool {
    check_walk(state, piece, move_to) && check_capture(state, piece, move_to)
}

/// Checks if the movement is legal
/// Ignores captures (or blockages)
pub fn check_walk(state: &State, piece: i32, move_to: i32) -> bool {
    let original = if state.cur_blue {
        state.board.blue[piece as usize]
    } else {
        state.board.red[piece as usize]
    };

    // Checks if out-of-bounds
    if original < 0 || move_to < 0 || original > 62 || move_to > 62 {
        return false;
    }

    // Checks if loop-around
    if (original % COL_COUNT == 0 && (move_to + 1) % COL_COUNT == 0)
        || (move_to % COL_COUNT == 0 && (original + 1) % COL_COUNT == 0)
    {
        return false;
    }

    // Checks if moving into one's own den
    if state.cur_blue {
        if piece == DEN_BLUE {
            return false;
        }
    } else if piece == DEN_RED {
        return false;
    }

    // Checks if cross-border (excpet for mouse)
    if piece != 0 && (RIVERS.contains(&move_to) && !RIVERS.contains(&original))
        || (RIVERS.contains(&original) && !RIVERS.contains(&move_to))
    {
        return false;
    }

    // // Checks if illegal horizontal 1-block move (except river move)
    // let mut river = false;
    // if (original - move_to).abs() != COL_COUNT {
    //     if [0, COL_COUNT - 1].contains(&(original % COL_COUNT))
    //         && [0, COL_COUNT - 1].contains(&(move_to % COL_COUNT))
    //     {
    //         return false;
    //     } else if (original - move_to).abs() != 1 {
    //         river = true;
    //     }
    // }
    //
    // let river_moves = [
    //     (15, 50),
    //     (16, 51),
    //     (18, 53),
    //     (19, 54),
    //     (21, 24),
    //     (28, 31),
    //     (35, 38),
    //     (24, 27),
    //     (31, 34),
    //     (38, 41),
    // ];
    // // Checks if non-river moves
    // if river
    //     && !river_moves.contains(&(original, move_to))
    //     && !river_moves.contains(&(move_to, original))
    // {
    //     return false;
    // }

    // TODO:
    // Check for rat in river

    true
}

/// Checks if capture, if any, is legal (or crash into ally)
pub fn check_capture(state: &State, piece: i32, move_to: i32) -> bool {
    // Check if there is anything to capture
    // or crash into ally
    if state.cur_blue {
        if state.board.blue.contains(&move_to) {
            return false;
        } else if !state.board.red.contains(&move_to) {
            return true;
        }
    } else if state.board.red.contains(&move_to) {
        return false;
    } else if !state.board.blue.contains(&move_to) {
        return true;
    }

    // Get enemy piece
    let enemy = if state.cur_blue {
        state.board.red.iter().position(|&x| x == move_to).unwrap() as i32
    } else {
        state.board.blue.iter().position(|&x| x == move_to).unwrap() as i32
    };

    // Check if cross-border
    if (RIVERS.contains(&move_to) && !RIVERS.contains(&enemy))
        || (RIVERS.contains(&enemy) && !RIVERS.contains(&move_to))
    {
        return false;
    }

    // Check capture
    if (piece >= enemy && !(piece == 7 && enemy == 0)) || piece == 0 && enemy == 7 {
        true
    } else if state.cur_blue {
        TRAPS_BLUE.contains(&enemy)
    } else {
        TRAPS_RED.contains(&enemy)
    }
}

/// Assumes that the move is legal already
pub fn make_move(state: &State, piece: i32, move_to: i32) -> State {
    let mut new_state = *state;

    // Move piece and make capture if needed
    if new_state.cur_blue {
        new_state.board.blue[piece as usize] = move_to;
        if state.board.red.contains(&move_to) {
            let enemy = state.board.red.iter().position(|&x| x == move_to).unwrap();
            new_state.board.red[enemy] = 63;
        }
    } else {
        new_state.board.red[piece as usize] = move_to;
        if state.board.blue.contains(&move_to) {
            let enemy = state.board.blue.iter().position(|&x| x == move_to).unwrap();
            new_state.board.blue[enemy] = 63;
        }
    }

    // Toggles switches
    new_state.cur_blue = !new_state.cur_blue;
    new_state.won = check_win(&new_state);

    new_state
}

pub fn check_win(state: &State) -> bool {
    state.board.blue.contains(&DEN_RED)
        || state.board.red.contains(&DEN_BLUE)
        || state.board.blue.iter().all(|x| x > &62)
        || state.board.red.iter().all(|x| x > &62)
}
