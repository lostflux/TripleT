
use crate::game::{Move, Game};

pub trait Player {
  fn get_move(&self, game: &mut Game) -> Option<Move>;
}

pub mod random_player;
pub mod minimax_player;
