
use crate::game::Game;
use crate::game_move::Move;
use super::{
    GameState,
    moves::Piece,
    MoveType,
    state::{
        BOARD_WIDTH,
        FINAL_POSITION,
        board_restore,
    },
};

use hdk::holochain_persistence_api::cas::content::Address;

impl Move {
    pub fn is_valid(&self, game: Game, game_state: GameState) -> Result<(), String> {
        is_playing(&self.author.clone(), &game)?;
        is_players_turn(self.author.clone(), &game, &game_state)?; // early return with error
        match &self.move_type {
            MoveType::Place{ pos } => {
                // let pos = Piece { pile, n };
                pos.is_in_bounds()?;
                pos.is_allowed_number()?;
                pos.is_not_empty(&game_state)?;
                Ok(())
            }
        }
    }
}

// HELPER FUNCTIONS

// pub enum Player {
//     Player1, 
//     Player2, 
// }

// pub fn _get_current_player(game: &Game, player_address: &Address) -> Result<Player, String> {
//     match (player_address == &game.player_1, player_address == &game.player_2) {
//         (true, false) => Ok(Player::Player1),
//         (false, true) => Ok(Player::Player2),
//         (true, true) => return Err("Player cannot play themselves".into()),
//         (false, false) => return Err("Player is not part of this game!".into()),
//     }
// }

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

// check that player is part of the game
fn is_playing(player_address: &Address, game: &Game) -> Result<(), String> {
    match (player_address == &game.player_1, player_address == &game.player_2) {
        (true, false) => Ok(()),
        (false, true) => Ok(()),
        (true, true) => Err("Player cannot play themselves".into()),
        (false, false) => Err("Player is not part of this game!".into()),
    }
}


impl Piece {
    pub fn is_in_bounds(&self) -> Result<(), String> {
        if self.pile < BOARD_WIDTH {
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