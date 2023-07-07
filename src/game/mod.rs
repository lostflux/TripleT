//? Game State and Game State Manager

use std::fmt::Display;
use rand::seq::SliceRandom;

pub type Move = (usize, usize);

pub struct Game {
  state: Vec<Vec<usize>>,
  current_player: usize,
  pub is_over: bool,
  pub winner: Option<usize>,
}

impl Game {
  pub fn new(dim: usize) -> Game {
    Game {
      state: vec![vec![0; dim]; dim],
      current_player: 1,
      is_over: false,
      winner: None,
    }
  }

  pub fn print(&self) {
    for row in &self.state {
      println!("{:?}", row);
    }
  }
  
  pub fn get_moves(&self) -> Vec<Move> {
    let mut moves = Vec::new();
    for (i, row) in self.state.iter().enumerate() {
      for (j, col) in row.iter().enumerate() {
        if *col == 0 {
          moves.push((i, j));
        }
      }
    }
    moves
  }

  pub fn toggle_player(&mut self) {
    self.current_player = 3 - self.current_player;
  }

  pub fn get_random_move(&self) -> Move {
    let moves = self.get_moves();
    let mut rng = rand::thread_rng();
    *moves.choose(&mut rng).unwrap()
  }

  fn update_state(&mut self, new_move: Move) {
    // check if new move is a winning move or the game is over
    let is_winning_move: bool = {
      let mut row = true;
      let mut col = true;
      let mut diag = true;
      let mut rdiag = true;
      let n = self.state.len();
      for i in 0..n {
        row &= self.state[new_move.0][i] == self.current_player;
        col &= self.state[i][new_move.1] == self.current_player;
        diag &= self.state[i][i] == self.current_player;
        rdiag &= self.state[i][n - i - 1] == self.current_player;
      }
      row || col || diag || rdiag
    };

    if is_winning_move {
      self.is_over = true;
      self.winner = Some(self.current_player);
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
  pub fn make_move(&mut self, new_move: Move) {
    if self.get_moves().contains(&new_move) {
      self.state[new_move.0][new_move.1] = self.current_player;
      self.update_state(new_move);
      self.toggle_player();
    } else {
      panic!("Invalid move!")
    }
  }

  pub fn is_active(&self) -> bool {
    !self.is_over
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
          0 => write!(f, "- "),
          1 => write!(f, "X "),
          2 => write!(f, "O "),
          _ => write!(f, "Invalid value in game state!"),
        }?;
      }
      write!(f, "\n")?;
    }

    // if game is over, print winner.
    if self.is_over {
      write!(f, "Game over!")?;
      match &self.winner {
        Some(1) => write!(f, " X wins!"),
        Some(2) => write!(f, " O wins!"),
        Some(_) => write!(f, "Invalid winner!"),
        None => write!(f, "It's a draw!"),
      }?;
      write!(f, "\n")?;
    }
    Ok(())
  }
}
