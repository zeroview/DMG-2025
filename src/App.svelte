<script lang="ts">
  import { run } from "DMG-2025";

  let started = false;

  function run_emulator() {
    started = true;

    let proxy = run();
    proxy.send("hell");

    let lastTime = performance.now();
    function frame() {
      let currentTime = performance.now();
      let millis = Math.min(100, Math.max(0, currentTime - lastTime));
      console.log(millis);
      lastTime = currentTime;

      proxy.run_cpu(millis);
      window.requestAnimationFrame(frame);
    }
    window.requestAnimationFrame(frame);
  }
</script>

<main>
  <canvas id="canvas"></canvas>
  {#if !started}
    <button on:click={run_emulator}> Run emulator </button>
  {/if}
</main>
