
use crate::game::Game;
use crate::game_move::Move;
use super::{
    GameState,
    moves::Piece,
    MoveType,
    state::{
        BOARD_SIZE,
        // board_sparse_to_dense, 
    },
};
use crate::your_game::MoveType;
use hdk::holochain_core_types::cas::content::Address;

/**
 *
 * To implement your own custom rule validation all you need to do is re-implement the function `is_valid` on `Move`
 * 
 * This function  takes the current game and the game state (which includes all the existing moves) 
 * and determines if a new candidate move is valid. Typically this will involve first matching on the move type
 * and then determining if the move is valid.
 * 
 * It function must return Ok(()) if a move is valid and Err("Some error string") for an invalid move.
 * It is useful to provide descriptive error strings as these can be visible to the end user.
 *
 */

impl Move {
    pub fn is_valid(&self, game: Game, game_state: GameState) -> Result<(), String> {
        // <<DEVCAMP-TODO>> Check if a move is valid given the current game and its state
        is_players_turn(self.author.clone(), &game, &game_state)?; // early return with error
        match self.move_type {
            MoveType::Place{ pile, n } => {
                let pos = Piece { pile, n };
                pos.is_in_bounds()?;
                pos.is_allowed_number()?;
                pos.is_not_empty(&game_state)?;
                Ok(())
            }
        }
    }
}

// HELPER FUNCTIONS

pub enum Player {
    Player1, 
    Player2, 
}

pub fn get_current_player(game: &game, player_address: &Address) -> Result<Player, String> {
    match (player_address == &game.player_1, player_address == &game.player_2) {
        (true, false) => Ok(Player::Player1),
        (false, true) => Ok(Player::Player2),
        (true, true) => return Err("Player cannot play themselves".into()),
        (false, false) => return Err("Player is not part of this game!".into()),
    }
}


fn is_players_turn(player: Address, game: &Game, game_state: &GameState) -> Result<(), String> {
    let moves = &game_state.moves;
    match moves.last() {
        Some(last_move) => {
            if last_move.author == player { 
                Err("It is not this players turn".into())
            } else {
                Ok(())
            }
        },
        None => {
            if game.player_2 == player {
                Ok(())
            } else {
                Err("Player 2 must make the first move".into())
            }
        }
    }
}

// TODO: seems like this one is useless 
// if I properly rewrite code - not
impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.pile == other.pile && self.n == other.n
    }
}

impl Piece {
    pub fn is_in_bounds(&self) -> Result<(), String> {
        if self.pile >= 0 && self.pile < BOARD_SIZE {
            Ok(())
        } else {
            Err("Piece is not in bounds".into())
        }
    }
    
    pub fn is_not_empty(&self, game_state: &GameState) -> Result<(), String> {
        match (board_restore(game_state)[self.pile] + self.n) <= FINAL_POSITION[self.pile] {
            true => Ok(()),
            false => Err("Too many pieces for this pile".to_string())
        }
    }

    pub fn is_allowed_number(&self) -> Result<(), String> {
        if self.n > 0 {
            Ok(())
        } else {
            Err("Number of pieces shoulb be > 0".into())
        }
    }
}