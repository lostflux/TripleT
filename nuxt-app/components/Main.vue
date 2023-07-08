<template>
  <div>
    <NuxtWelcome />
    <div>
      This is a test.
    </div>
  </div>
</template>
<script lang="ts">
import init, { Game } from '../../pkg/ttt';

const run = async () => {
  await init();
  const game = Game.new(4);
  console.log(game);
}

run();

</script>

<style lang="sass" scoped>
</style>
