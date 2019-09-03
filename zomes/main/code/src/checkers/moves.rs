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
pub struct Piece {
     x: u32,
     y: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub enum MoveType {
    /**
     * DEVCAMP TODO #1: 
     * Input all possible move types of your game
     * 
     * Hint: Enum variants can be structs, and thus have parameters
     * References: https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
     * Examples: 
     *     RollDice { number: u32 }
     *     GuessMovie(String)
     */
    MovePiece { from: Piece, to: Piece },
    Resign
}

impl MoveType {
	pub fn describe() -> Vec<MoveType> {
        /**
         * DEVCAMP TODO #2:
         * Return a list containing each one of the MoveType enum variants
         * This is only a helper list function for the CLI to be able to output the list of possible moves
         * 
         * Hint: use the vec![] macro
         * References: https://doc.rust-lang.org/1.3.0/std/macro.vec!.html
         */
        vec![MoveType::MovePiece { from: Piece { x: 0, y: 0 }, to: Piece { x: 0, y: 0 } }, MoveType::Resign]
	}
}