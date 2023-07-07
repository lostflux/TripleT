use ttt::game::{Game, Move};

fn main() {
  println!("Hello, world!");

  let mut game = Game::new(3);
  //game.print();

  println!("{}", game);

  let moves = game.get_moves();
  println!("{:?}", moves);

  game.make_move((0, 0));

  game.make_move((1, 1));

  game.make_move((0, 1));

  println!("{}", game);

  println!("{:?}", game.get_moves());

  let mut game = Game::new(4);

  println!("{}", game);

  while game.is_active() {
    let random_move: Move = game.get_random_move();

    println!("Making move: {:?}", random_move);

    game.make_move(random_move);
    
    println!("{}", game);
  }
}
