
use super::*;

use crate::game::{
    Move
  , Game
  , Mark::*
};

pub struct MinimaxPlayer {
  pub depth: usize,
  pub maximizing: bool
}

impl MinimaxPlayer {

  pub fn new(depth: usize, maximizing: bool) -> MinimaxPlayer {
    MinimaxPlayer { depth, maximizing }
  }

  pub fn minimax(&self, game: &mut Game, current_depth: usize) -> i32 {

    if game.is_over {
      return match game.winner {
        Empty => 0,
        X => match self.maximizing {
          true => std::i32::MAX,
          false => std::i32::MIN
        },
        O => match self.maximizing {
          true => std::i32::MIN,
          false => std::i32::MAX
        }
      }
    } else if current_depth == self.depth {
      return game.evaluate();
    }

    let turn = current_depth % 2;
    let mut best_score: i32 = match self.maximizing {
      true => match turn {
        0 => std::i32::MIN,
        1 => std::i32::MAX,
        _ => panic!("Invalid turn!")
      },
      false => match turn {
        0 => std::i32::MAX,
        1 => std::i32::MIN,
        _ => panic!("Invalid turn!")
      }
    };

    let moves = game.get_moves();
    for m in moves {
      game.make_move(m);
      let score = self.minimax(game, current_depth + 1);
      game.undo_move(m);
      match self.maximizing {
        true => {
          if score > best_score {
            best_score = score;
          }
        },
        false => {
          if score < best_score {
            best_score = score;
          }
        }
      }
    }
    best_score
  }
}

#[macro_export]
macro_rules! minimax_player {
  ($depth:expr) => {
    MinimaxPlayer::new($depth, false)
  };
  ($depth:expr, $maximizing:expr) => {
    MinimaxPlayer::new($depth, $maximizing)
  };
}

impl Player for MinimaxPlayer {
  fn get_move(&self, game: &mut Game) -> Option<Move> {
    let mut best_move: Option<Move> = None;
    let mut best_score: i32 = match self.maximizing {
      true => std::i32::MIN,
      false => std::i32::MAX
    };
    let moves = game.get_moves();
    for m in moves {
      game.make_move(m);
      let score = self.minimax(game, 0);
      game.undo_move(m);
      match self.maximizing {
        true => {
          if score > best_score {
            best_score = score;
            best_move = Some(m);
          }
        },
        false => {
          if score < best_score {
            best_score = score;
            best_move = Some(m);
          }
        }
      }
    }
    best_move
  }
}
