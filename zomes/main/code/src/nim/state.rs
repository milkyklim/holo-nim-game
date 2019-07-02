use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};
use hdk::AGENT_ADDRESS;

use crate::game_move::Move;
use crate::game::Game;
use super::{
    MoveType, 
    moves::Piece, 
    // validation::{
    //     // Player, 
    //     // get_current_player,
    // },
};

pub const EMPTY_SPACE: char = ' ';
pub const PIECE: char = 'x';
// final state of the game
pub const FINAL_POSITION: [usize; 3] = [3, 4, 5];
pub const BOARD_WIDTH: usize = 3;
pub const BOARD_HEIGHT: usize = 5;

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct GameState {
    pub moves: Vec<Move>,
    pub player_1: PlayerState,
    pub player_2: PlayerState,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct PlayerState {
    pub pieces: Vec<Piece>,
    pub resigned: bool,
    pub winner: bool,
}

impl PlayerState {
    pub fn initial() -> Self {
        PlayerState {
            pieces: Vec::new(),
            resigned: false,
            winner: false,
        }
    }
}

impl GameState {
    pub fn initial() -> Self {
        GameState {
            moves: Vec::new(),
            player_1: PlayerState::initial(),
            player_2: PlayerState::initial(),
        }
    }

    pub fn render(&self) -> String {
        let mut disp = "\n".to_string();

        if let Some(last_move) = self.moves.last() {
            if last_move.author.to_string() == AGENT_ADDRESS.to_string() {
                disp.push_str("It is your opponents turn \n");
            } else {
                disp.push_str("It is your turn \n");
            }
        } else {
            disp.push_str("Non-creator must make the first move \n");        
        }

        disp.push('\n');
        disp.push_str("  p  0 1 2\nn\n");

        let board = board_restore_visual(&self);

        for y in 0..BOARD_HEIGHT {
            disp.push_str(&format!("{}   |", BOARD_HEIGHT - 1 - y));
            for x in 0..BOARD_WIDTH {
                let c = match board[BOARD_HEIGHT - 1 - y][x] {
                    0 => EMPTY_SPACE,
                    _ => PIECE,
                };
                disp.push_str(&format!("{}|", c));
            }
            disp.push('\n');
        }

        if self.player_1.resigned {
            disp.push_str(&format!("Game over: Player 1 has resigned!\n"));
        } else if self.player_2.resigned {
            disp.push_str(&format!("Game over: Player 2 has resigned!\n"));
        } else if self.player_1.winner {
            disp.push_str(&format!("Game over: Player 1 is the winner!\n"));
        } else if self.player_2.winner {
            disp.push_str(&format!("Game over: Player 2 is the winner!\n"));
        }
        disp
    }

    pub fn evolve(&self, game: Game, next_move: &Move) -> Self {        
        let mut moves = self.moves.clone();
        moves.push(next_move.to_owned());

        match &next_move.move_type {
            MoveType::Place{ pos } => {
                let mut board = board_restore(&self);
                // make a move
                board[pos.pile] += pos.n;
                // check if this is players victory
                let victory: bool = FINAL_POSITION.iter().fold(0,|a, &b| a + b) == board.iter().fold(0,|a, &b| a + b);
                // we want to return tuple of the form (bool, bool)
                let (player_1_victory, player_2_victory) = match &next_move.author == &game.player_1 {
                    true => (victory, false),
                    false => (false, victory),
                };

                // TODO: move the code to separate function
                // who put where and how many
                let player_1_pieces = &mut self.player_1.pieces.clone();
                let player_2_pieces = &mut self.player_2.pieces.clone();

                match &next_move.author == &game.player_1 {
                    true => player_1_pieces.push(pos.clone()),
                    false => player_2_pieces.push(pos.clone()),
                };

                GameState{
                    player_1: PlayerState {
                        pieces: player_1_pieces.to_vec(), //Vec::new(), // player_1_pieces,
                        resigned: false,
                        winner: player_1_victory,
                    },
                    player_2: PlayerState {
                        pieces: player_2_pieces.to_vec(), // Vec::new(), // player_2_pieces,
                        resigned: false,
                        winner: player_2_victory,
                    },
                    moves,
                    ..self.clone()
                }
            }
        }
    }
}

// Helper functions 
pub fn board_restore(state: &GameState) -> [usize; BOARD_WIDTH] {
    let mut board = [0usize; BOARD_WIDTH];
    state.player_1.pieces.iter().for_each(|p| { 
        board[p.pile] += p.n;
    });
    state.player_2.pieces.iter().for_each(|p| { 
        board[p.pile] += p.n;
    });
    board
}

// RUST notation: 
// const M: usize = 5; // rows
// const N: usize = 3; // columns
// let mut grid = [[0 as u8; N] ; M];

pub fn board_restore_visual(state: &GameState) -> [[usize; BOARD_WIDTH]; BOARD_HEIGHT] {
    let mut board = [[0usize; BOARD_WIDTH]; BOARD_HEIGHT];
    let mut tmp = [0usize; BOARD_WIDTH];

    // get the sum of pieces for each position
    state.player_1.pieces.iter().for_each(|p| { 
        tmp[p.pile] += p.n;
    });
    state.player_2.pieces.iter().for_each(|p| { 
        tmp[p.pile] += p.n;
    });

    for x in 0..BOARD_WIDTH {
        for y in 0..tmp[x]{
            board[y][x] = 1;
        }
    }
    board
}