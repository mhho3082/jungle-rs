// Note:
// Since many of the arrays we use here
// already have their lengths fixed,
// we use rust arrays `[x; n]`
// instead of vectors `vec![x; n]` for them.
// They function (mostly) the same,
// and we get some nice compiler checks.
//
// for their implementation:
// https://stackoverflow.com/questions/60583618/performance-of-rust-vector-vect-versus-array-t-n

/// The general model
#[derive(Debug)]
pub struct Model {
    /// The collection of all states played,
    /// including the empty state
    pub history: Vec<State>,
    pub current: usize,
}

impl Model {
    /// Initialize a new model
    pub fn new() -> Self {
        Model {
            history: vec![State::new()],
            current: 0,
        }
    }

    /// A shorthand for the current state
    pub fn curr(&self) -> &State {
        &self.history[self.current]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    /// The current board
    pub board: Board,
    /// If the current player is blue (player 1)
    pub cur_blue: bool,
    /// If the board is won at this point
    pub won: bool,
}

impl State {
    /// Gives the opening state
    pub fn new() -> Self {
        State {
            board: Board::new(),
            cur_blue: true,
            won: false,
        }
    }
}

/// The game board representation
///
/// Using row * colCount + col to create
/// a hidden 2-level vector's indices
/// The board is 9 rows * 7 cols
/// From 0 to 62, in total 63 squares
///
/// Each player has 8 pieces
/// stored in rank: 1 -> 8 (index 0 -> 7)
/// as their index (0..=62)
/// If stored as 63: captured
/// Can be formatted as R C D W O T L E
/// .                           ^- leopard
///
/// Blue is at bottom (as default player 1)
#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub blue: [i32; 8],
    pub red: [i32; 8],
}

impl Board {
    /// Gives the opening
    pub fn new() -> Self {
        Board {
            blue: [48, 50, 54, 44, 46, 56, 62, 42],
            red: [14, 12, 8, 18, 16, 6, 0, 20],
        }
    }
}

// Static values for controller
pub static RIVERS: [i32; 12] = [22, 23, 25, 26, 29, 30, 32, 33, 36, 37, 39, 40];
pub static TRAPS_BLUE: [i32; 3] = [52, 58, 60];
pub static TRAPS_RED: [i32; 3] = [2, 4, 10];
pub static DEN_BLUE: i32 = 59;
pub static DEN_RED: i32 = 3;
pub static COL_COUNT: i32 = 7;
pub static ROW_COUNT: i32 = 9;
