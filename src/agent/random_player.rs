

use super::*;

pub struct RandomPlayer;

impl RandomPlayer {
  pub fn new() -> RandomPlayer {
    RandomPlayer
  }
}

impl Player for RandomPlayer {
  fn get_move(&self, game: &mut Game) -> Option<Move> {
    game.get_random_move()
  }
}
