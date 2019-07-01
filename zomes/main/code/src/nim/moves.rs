use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};

/**
 *
 * The MoveType enum defines all the types of moves that are valid in your game and the 
 * data they carry. In Checkers you can move a piece (MovePiece) from a location to another location.
 *
 */

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub enum MoveType {
    // <<DEVCAMP-TODO>> YOUR MOVE ENUM VARIENTS HERE
	Place { 
		pos: Piece,
	}
}

impl MoveType {
	pub fn describe() -> Vec<MoveType> {
		// <<DEVCAMP-TODO>> SHOULD RETURN AN EXAMPLE OF EACH VARIENT
		vec![
			MoveType::Place{pos: Piece{pile:0, n: 1}}
		]
	}
}

// #[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub struct Piece {
    pub pile: usize,
    pub n: usize,
}

// TODO: seems like this one is useless 
// if I properly rewrite code - not
// impl PartialEq for Piece {
//     fn eq(&self, other: &Self) -> bool {
//         self.pile == other.pile && self.n == other.n
//     }
// }