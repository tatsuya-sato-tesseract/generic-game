use hdk::holochain_core_types::{
    error::HolochainError,
    json::JsonString,
};

use crate::game_move::Move;
use crate::game::Game;
use super::MoveType;

/**
 *
 * As a game author you get to decide what the State object of your game looks like.
 * Most of the time you want it to include all of the previous moves as well.
 * 
 * To customize the game state implement your own GameState struct. 
 * This must have a function called `initial()`
 * which returns the initial state.
 *
 */


#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct GameState {
    pub moves: Vec<Move>,
    pub player_1_moves: Vec<Piece>,
    pub player_2_moves: Vec<Piece>,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Piece {
    pub x: usize,
    pub y: usize,
}


impl GameState {
    pub fn initial() -> Self {
        Self{
            moves: Vec::new(),
            player_1_moves: Vec::new(),
            player_2_moves: Vec::new(),
        }
    }

    pub fn render(&self) -> String {
        // <<DEVCAMP-TODO>> return a pretty formatting string representation
        "".to_string()
    }

    pub fn evolve(&self, game: Game, next_move: &Move) -> GameState {
        // <<DEVCAMP-TODO>>
        // given a current state, a game and a move, compute the next state
        // You can assume all moves are valid
        
        // unpack the move
        let mut moves = self.moves.clone();
        let mut player_1_moves = self.player_1_moves.clone();
        let mut player_2_moves = self.player_2_moves.clone();

        // add the new move to the state
        moves.push(next_move.clone());

        match next_move.move_type {
            MoveType::Place{x, y} => {
                // figure out which player made the move
                if game.player_1 == next_move.author {
                    player_1_moves.push(Piece{x, y});
                } else {
                    player_2_moves.push(Piece{x, y});
                }

                // finally return the new state
                GameState{
                    moves,
                    player_1_moves,
                    player_2_moves
                }
            }
        }
    }

}
