use ttt::{
    game::{Game, Move}
  , agent::{
      Player
    , random_player::RandomPlayer
    , minimax_player::MinimaxPlayer
  }, minimax_player
};

fn main() {
  simulate_game();
}

fn simulate_game() {
  let mut game = Game::new(4);
  let random_player = RandomPlayer::new();
  let minimax_player = minimax_player!(2, false);

  println!("{}", game);

  let mut moves = 0;
  while game.is_active() {
    
    let turn = moves % 2;

    let current_move = match turn {
      0 => random_player.get_move(&mut game),
      1 => minimax_player.get_move(&mut game),
      _ => panic!("Invalid turn!")
    };

    match current_move {
      Some(m) => {
        println!("Making move: {}", m);
        game.make_move(m);
        println!("{}", game);
      },
      None => panic!("Invalid move!")
    }

    moves += 1;
    
    println!("{}", game);
  }
}
