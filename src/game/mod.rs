//? Game State and Game State Manager

use std::fmt::Display;
use rand::seq::SliceRandom;
use crate::game::Mark::*;
use wasm_bindgen::prelude::*;

// pub type Move = (usize, usize);

// import everything from moves module
pub mod moves;
pub use moves::*;



#[derive(Clone, Copy, PartialEq)]
#[wasm_bindgen]
pub enum Mark {
  X,
  O,
  Empty,
}









#[wasm_bindgen]
pub struct Game {
  state: Vec<Vec<Mark>>,
  current_player: Mark,
  pub is_over: bool,
  pub winner: Mark,
  pub dim: usize,
}

// #[wasm_bindgen]
impl Game {

  /// Create a new game instance with given dimension.
  /// 
  /// The board will have dimension `dim` x `dim`.
  // #[wasm_bindgen(constructor)]
  pub fn new(dim: usize) -> Game {
    Game {
      state: vec![vec![Empty; dim]; dim],
      current_player: X,
      is_over: false,
      winner: Empty,
      dim,
    }
  }
  
  /// Get a list of all possible moves.
  /// 
  /// Returns a vector of tuples representing the coordinates of the empty cells.
  pub fn get_moves(&self) -> Vec<Move> {
    let mut moves = Vec::new();
    for i in 0..self.dim {
      for j in 0..self.dim {
        if self.get(i, j) == Empty {
          let new_move = Move::new(i, j);
          moves.push(new_move);
        }
      }
    }
    moves
  }

  /// Toggle the current player.
  /// 
  /// If the current player is X, it will be changed to O and vice versa.
  pub fn toggle_player(&mut self) {
    match self.current_player {
      X => self.current_player = O,
      O => self.current_player = X,
      Empty => panic!("Invalid player!"),
    }
  }

  /// Get a random move from the list of all possible moves.
  /// 
  /// Returns a tuple representing the coordinates of the empty cell.
  pub fn get_random_move(&self) -> Move {
    let moves = self.get_moves();
    let mut rng = rand::thread_rng();
    *moves.choose(&mut rng).unwrap()
  }

  /// Given a new move, applies the move to the board and updates the game state.
  /// 
  /// It also updates metrics such as if the game is over and who is the winner.
  fn update_state(&mut self, new_move: Move) {
    self.set(new_move, self.current_player.clone());
    let is_winning_move: bool = {
      let mut row = true;
      let mut col = true;
      let mut diag = true;
      let mut rdiag = true;
      let n = self.state.len();
      for i in 0..n {
        row &= self.state[new_move.x][i] == self.current_player;
        col &= self.state[i][new_move.y] == self.current_player;
        diag &= self.state[i][i] == self.current_player;
        rdiag &= self.state[i][n - i - 1] == self.current_player;
      }
      row || col || diag || rdiag
    };

    if is_winning_move {
      self.is_over = true;
      self.winner = self.current_player.clone();
    } else if self.get_moves().is_empty() {
      self.is_over = true;
    }
  }

  /// Make a move on the board
  /// 
  /// # Arguments
  /// new_move: a tuple representing the coordinates of the move.
  /// 
  /// # Panics
  /// Panics if the move is invalid.
  /// 
  /// # Examples
  /// ```
  /// use ttt::game::Game;
  /// 
  /// let mut game = Game::new(3);
  /// println!("{}", game);
  /// game.make_move((0, 0));
  /// println!("{}", game);
  /// ```
  /// 
  pub fn make_move(&mut self, new_move: Move) {
    if self.is_valid_move(new_move) {
      self.update_state(new_move);
      self.toggle_player();
    } else {
      panic!("Invalid move!")
    }
  }

  /// Check if a move is valid.
  /// 
  /// # Examples
  /// ```
  /// use ttt::game::Game;
  /// 
  /// let mut game = Game::new(3);
  /// assert!(game.is_valid_move((0, 0)));
  /// game.make_move((0, 0));
  /// assert!(!game.is_valid_move((0, 0)));
  /// ```
  pub fn is_valid_move(&self, new_move: Move) -> bool {
    self.is_active()
      && new_move.x < self.dim
      && new_move.y < self.dim
      && self.get(new_move.x, new_move.y) == Empty
  }

  /// Check if the game is active,
  /// i.e. if the game is not over.
  /// 
  /// A game is over if a player has won or if there are no more moves left.
  pub fn is_active(&self) -> bool {
    !self.is_over
  }

  /// Get the mark at a given position.
  fn get(&self, i: usize, j: usize) -> Mark {
    self.state[i][j].clone()
  }

  /// function that takes a move and a mark and sets the state of the game.
  /// 
  /// It can also take the individual coordinates.
  pub fn set(&mut self, new_move: Move, mark: Mark) {
    let i = &new_move.x;
    let j = &new_move.y;
    self.state[*i][*j] = mark;
  }
}

impl Display for Game {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "\n")?;
    for row in &self.state {
      for col in row {
        // if col value is zero, print dash.
        // if col value is 1, print X.
        // if col value is 2, print O.
        match col {
          Empty => write!(f, "- "),
          X     => write!(f, "X "),
          O     => write!(f, "O "),
        }?;
      }
      write!(f, "\n")?;
    }

    // if game is over, print winner.
    if self.is_over {
      writeln!(f, "Game over!")?;
      match &self.winner {
        X => write!(f, " X wins!"),
        O => write!(f, " O wins!"),
        Empty => writeln!(f, "It's a draw!"),
      }?;
      write!(f, "\n")?;
    }
    Ok(())
  }
}
