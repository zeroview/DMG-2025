<script lang="ts">
  import EmulatorManager from "./manager.svelte";
  let { manager }: { manager: EmulatorManager } = $props();
  import homebrewRoms from "../roms/homebrewhub.json";

  interface GameInfo {
    developer: string;
    typetag: string;
    download_url: string;
    image_url: string;
  }
  const roms = homebrewRoms as unknown as Record<string, GameInfo>;
  let romTitles = Object.keys(roms);
  function load(url: string) {
    fetch(url).then((response) => {
      response.arrayBuffer().then((buffer) => {
        manager.loadRom(buffer, false);
      });
    });
  }
</script>

<div class="browser-list">
  {#each romTitles as title}
    <div class="browser-item">
      {#if roms[title].image_url !== ""}
        <div class="browser-img">
          <img src={roms[title].image_url} alt={title} />
        </div>
      {/if}

      <button onclick={() => load(roms[title].download_url)}>{title}</button>
      <p>{roms[title].developer}</p>
    </div>
  {/each}
</div>
