

import init, { Game } from '../pkg/ttt';

const run = async () => {
  await init();
  const game = new Game();
  console.log(game);
}

run();
