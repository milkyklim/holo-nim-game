use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};

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

pub const BOARD_SIZE: usize = 3;
pub const _EMPTY_SPACE: char = ' ';
pub const _PIECE: char = 'x';
// final state of the game
pub const FINAL_POSITION: [usize; 3] = [3, 4, 5];

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
        // <<DEVCAMP>> return a pretty formatting string representation
        "".to_string()
        // let mut disp = "\n".to_string();

        // if let Some(last_move) = self.moves.last() {
        //     if last_move.author.to_string() == AGENT_ADDRESS.to_string() {
        //         disp.push_str("It is your opponents turn \n");
        //     } else {
        //         disp.push_str("It is your turn \n");
        //     }
        // } else {
        //     disp.push_str("Non-creator must make the first move \n");        
        // }

        // disp.push('\n');

        // let board = board_restore(self);

    }

    pub fn evolve(&self, game: Game, next_move: &Move) -> Self {
        // let current_player = get_current_player(&game, &next_move.author).unwrap();
        
        // let mut moves = self.moves.clone();
        // moves.push(next_move.to_owned());

        match &next_move.move_type {
            MoveType::Place{ pos } => {
                let mut board = board_restore(&self);
                let mut moves = self.moves.clone();
                moves.push(next_move.to_owned());

                // make a move
                board[pos.pile] += pos.n;
                // check if this is players victory
                // let victory: bool = FINAL_POSITION.iter().sum() == board.iter().sum();

                let victory: bool = FINAL_POSITION.iter().fold(0,|a, &b| a + b) == board.iter().fold(0,|a, &b| a + b);
                // we want to return tuple of the form (bool, bool)
                let (player_1_victory, player_2_victory) = match &next_move.author == &game.player_1 {
                    true => (victory, false),
                    false => (false, victory),
                };



                GameState{
                    player_1: PlayerState {
                        pieces: Vec::new(), // player_1_pieces,
                        resigned: false,
                        winner: player_1_victory,
                    },
                    player_2: PlayerState {
                        pieces: Vec::new(), // player_2_pieces,
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

pub fn board_restore(state: &GameState) -> [usize; 3] {
    let mut board = [0usize; 3];
    state.player_1.pieces.iter().for_each(|p| { 
        board[p.pile] += p.n;
    });
    state.player_2.pieces.iter().for_each(|p| { 
        board[p.pile] += p.n;
    });
    board
}
