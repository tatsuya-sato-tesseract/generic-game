use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};
use hdk::AGENT_ADDRESS;

use crate::game_move::Move;
use crate::game::Game;
use super::{
    MoveType,
    // Usually you would import structs and types defined in the 'moves' file
    Piece
};

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
    /**
     * DEVCAMP TODO #3:
     * Implement struct that determines the state of your game
     * 
     * Hint: you can define other helper structs and reuse the Move struct to store the state
     * References: https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
     */
    moves: Vec<Move>,
    player_1: PlayerState,
    player_2: PlayerState,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct PlayerState { 
    pieces: Vec<Piece>,
    resigned: bool,
    winner: bool,
}



/**
 * Example of a possible GameState
 * 
 * #[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
 * pub struct GameState {
 *     pub moves: Vec<Move>,
 *     pub player_1: PlayerState,
 *     pub player_2: PlayerState,
 * }
 * 
 * #[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
 * pub struct PlayerState {
 *     pub pieces: Vec<Piece>,
 *     pub resigned: bool,
 *     pub winner: bool,
 * }
 */

impl PlayerState {
    pub fn initial_black() -> Self {
        PlayerState {
            pieces: vec![
                Piece {
                    x: 0,
                    y: 0,
                },
                Piece {
                    x: 2,
                    y: 0,
                },
                Piece {
                    x: 4,
                    y: 0,
                },
                Piece {
                    x: 6,
                    y: 0,
                },
                Piece {
                    x: 1,
                    y: 1,
                },
                Piece {
                    x: 3,
                    y: 1,
                },
                Piece {
                    x: 5,
                    y: 1,
                },
                Piece {
                    x: 7,
                    y: 1,
                },
                Piece {
                    x: 0,
                    y: 2,
                },
                Piece {
                    x: 2,
                    y: 2,
                },
                Piece {
                    x: 4,
                    y: 2,
                },
                Piece {
                    x: 6,
                    y: 2,
                },
            ],
            winner: false,
            resigned: false
        }
    }
    
    pub fn initial_white() -> Self {
        PlayerState {
            pieces: vec![
                Piece {
                    x: 1,
                    y: 5,
                },
                Piece {
                    x: 3,
                    y: 5,
                },
                Piece {
                    x: 5,
                    y: 5,
                },
                Piece {
                    x: 7,
                    y: 5,
                },
                Piece {
                    x: 0,
                    y: 6,
                },
                Piece {
                    x: 2,
                    y: 6,
                },
                Piece {
                    x: 4,
                    y: 6,
                },
                Piece {
                    x: 6,
                    y: 6,
                },
                Piece {
                    x: 1,
                    y: 7,
                },
                Piece {
                    x: 3,
                    y: 7,
                },
                Piece {
                    x: 5,
                    y: 7,
                },
                Piece {
                    x: 7,
                    y: 7,
                },
            ],
            winner: false,
            resigned: false
        }
    }

}

impl GameState {
    pub fn initial() -> Self {
        /**
         * DEVCAMP TODO #4:
         * Return the initial game state
         * 
         * Hint: 
         * References: 
         */
        GameState {
            moves: Vec::new(),
            player_1: PlayerState::initial_white(),
            player_2: PlayerState::initial_black()
        }
    }

    pub fn render(&self) -> String {
        /**
         * DEVCAMP TODO #5:
         * Return a string rendering the state of the game, so that the CLI can render it in the terminal
         * 
         * Hint: useful snippets:
         *     let mut disp = "\n".to_string();
         *     disp.push_str("  x  0 1 2 3 4 5 6 7\ny\n");
         * References: 
         */
    }

    pub fn evolve(&self, game: Game, next_move: &Move) -> Self {
        /**
         * DEVCAMP TODO #6:
         * Return the new game state resulting from applying the next_move to the current state of the game
         * You can assume that next_move is valid
         * 
         * Hints: 
         *   - This is similar to a Redux reducer, a function that given a state and a new action,
         *     returns the next state
         *   - You can declare helper functions (eg: get_current_player, format_state)
         *     to help transform your game state into an easier format for this function to read
         * References: 
         */
        
        


        match next_move.move_type {

        }
          
    }
}


pub fn board_sparse_to_dense(state: &GameState)-> [[u8; 8]; 8] {
    let mut board = [[0u8; 8]; 8];
    state.player_1.pieces.iter().for_each(|piece| {
        board[piece.x][piece.y] = 1;
    });
    state.player_2.pieces.iter().for_each(|piece| {
        board[piece.x][piece.y] = 2;
    });
    board
}

pub fn board_dense_to_sparse(board: [[u8; 8]; 8]) -> (Vec<Piece>, Vec<Piece>) {
    let mut player_1_pieces = Vec::new();
    let mut player_2_pieces = Vec::new();
    board.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, square)| {
            if *square == 1 {
                player_1_pieces.push(Piece{x , y});
            } else if *square == 2 {
                player_2_pieces.push(Piece{x , y});               
            }
        })
    });
    (player_1_pieces, player_2_pieces)
}
