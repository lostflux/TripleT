use ttt::game::{Game, Move};

fn main() {

  let mut game = Game::new(4);

  println!("{}", game);

  while game.is_active() {
    let random_move: Move = game.get_random_move();

    println!("Making move: {}", random_move);

    game.make_move(random_move);
    
    println!("{}", game);
  }
}
