use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};

use crate::game_move::Move;
use crate::game::Game;
use super::MoveType;


/**
 *
 * As a game author you get to decide what the State object of your game looks like.
 * Most of the time you want it to include all of the previous moves as well.
 * 
 * To customize the game state implement your own GameState struct. This must have a function called `initial()`
 * which returns the initial state.
 *
 */


#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct GameState {
    // <<DEVCAMP-TODO>>
    pub moves: Vec<Move>,
    // Implement your own game state
    // May be helpful to split this into state for each player

    pub player_1_moves: Vec<Piece>,
    pub player_2_moves: Vec<Piece>,

}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Piece {
    pile: usize,
    n: usize,
}


impl GameState {
    pub fn initial() -> Self {
        // <<DEVCAMP>> return an initial state of a game
        Self{
            moves: Vec::new(),
            player_1_moves: Vec::new(),
            player_2_moves: Vec::new(),
        }
    }

    pub fn render(&self) -> String {
        // <<DEVCAMP>> return a pretty formatting string representation
        "".to_string()
    }

    pub fn evolve(&self, game: Game, next_move: &Move) -> GameState {
        // <<DEVCAMP>>
        // given a current state, a game and a move, compute the next state
        // You can assume all moves are valid
        // self.clone()

        let mut moves = self.moves.clone();
        let mut player_1_moves = self.player_1_moves.clone();
        let mut player_2_moves = self.player_2_moves.clone();

        moves.push(next_move.clone());

        match next_move.move_type {
            MoveType::Place{pile, n} => {
                if game.player_1 == next_move.author{
                    player_1_moves.push(Piece{pile, n});
                } else {
                    player_2_moves.push(Piece{pile, n});
                }
            }
            GameState {
                moves, 
                player_1_moves,
                player_2_moves
            }
        }

    }

}
