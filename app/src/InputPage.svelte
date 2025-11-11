<script lang="ts">
  import InputManager, { Keybind } from "./input.svelte";
  let { input }: { input: InputManager } = $props();
</script>

{#snippet inputList(mappings: Keybind<string>[])}
  {#each mappings as mapping (mapping.name)}
    <p>{mapping.name}</p>
    {#if input.mappingToRebind == mapping.name}
      <button style="color:grey">[ Rebinding... ]</button>
    {:else}
      <button onclick={() => (input.mappingToRebind = mapping.name)}
        >{mapping.input}</button
      >
    {/if}
  {/each}
{/snippet}

<div class="menu-grid" style="grid-template-columns: 10rem 15rem">
  <h3>Controls</h3>
  {@render inputList(input.controls)}
  <h3>Keybinds</h3>
  {@render inputList(input.keybinds)}
</div>
