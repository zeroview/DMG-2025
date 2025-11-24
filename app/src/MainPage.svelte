<script lang="ts">
  import { defaultOptions, type Options } from "./options.svelte";
  import MenuSlider from "./MenuSlider.svelte";

  let {
    options = $bindable(),
    onBrowse,
    onSaveState,
    onLoadState,
    onSaveSlotChange,
    onLoadRom,
    saveStateDisabled: saveDisabled,
    loadStateDisabled: loadDisabled,
    stateSlot: saveSlot,
  }: {
    options: Options;
    onBrowse: () => void;
    onSaveState: () => void;
    onLoadState: () => void;
    onLoadRom: (rom: ArrayBuffer, isZip: boolean) => void;
    onSaveSlotChange: (change: number) => void;
    saveStateDisabled: boolean;
    loadStateDisabled: boolean;
    stateSlot: number;
  } = $props();

  const zipMimeTypes = [
    "application/zip",
    "application/x-zip-compressed",
    "application/x-zip",
  ];
  let files: FileList | undefined = $state();
  $effect(() => {
    // Open selected file as byte array
    if (files) {
      let file = files[0];
      // Determine file type
      let isZip = zipMimeTypes.includes(file.type);
      files[0].arrayBuffer().then((rom) => onLoadRom(rom, isZip));
    }
  });

  const speedSliderValues = [
    0.01, 0.05, 0.1, 0.3, 0.5, 0.7, 0.8, 0.9, 1, 1.1, 1.3, 1.5, 2, 3, 5, 10, 20,
  ];
</script>

<input
  id="fileInput"
  accept=".gb,.zip"
  type="file"
  bind:files
  style="display: none"
/>
<div class="button-row">
  <div class="img-button">
    <!-- Load icon -->
    <svg
      version="1.1"
      xmlns="http://www.w3.org/2000/svg"
      preserveAspectRatio="xMinYMin meet"
      viewBox="0 0 8 12"
    >
      <rect x="3" y="1" width="2" height="6" />
      <rect x="1" y="4" width="2" height="1" />
      <rect x="5" y="4" width="2" height="1" />
      <rect x="2" y="5" width="1" height="1" />
      <rect x="5" y="5" width="1" height="1" />
      <rect x="0" y="6" width="1" height="3" />
      <rect x="7" y="6" width="1" height="3" />
      <rect x="1" y="8" width="6" height="1" />
    </svg>
    <button onclick={() => document.getElementById("fileInput")?.click()}>
      Load ROM
    </button>
  </div>
  <div class="img-button">
    <!-- Browse icon -->
    <svg
      version="1.1"
      xmlns="http://www.w3.org/2000/svg"
      preserveAspectRatio="xMinYMin meet"
      viewBox="0 0 8 12"
    >
      <rect x="1" y="1" width="3" height="1" />
      <rect x="0" y="2" width="1" height="3" />
      <rect x="4" y="2" width="1" height="3" />
      <rect x="1" y="5" width="3" height="1" />
      <rect x="5" y="6" width="1" height="1" />
      <rect x="6" y="7" width="1" height="1" />
      <rect x="7" y="8" width="1" height="1" />
    </svg>
    <button onclick={() => onBrowse()}>Browse Homebrew Hub</button>
  </div>
</div>

<p style="height:1rem"></p>
<div class="button-row">
  <button onclick={onSaveState} disabled={saveDisabled}>Save state</button>
  <button onclick={onLoadState} disabled={loadDisabled}>Load state</button>
  <div class="button-row" style="gap:1rem">
    <p>Slot:</p>
    <button onclick={() => onSaveSlotChange(-1)}>&lt;</button>
    <p style="width:2rem; text-align: center">{saveSlot}</p>
    <button onclick={() => onSaveSlotChange(1)}>&gt;</button>
  </div>
</div>
<p style="height:1.5rem"></p>
<div class="menu-grid">
  <p>Volume:</p>
  <MenuSlider
    bind:value={options.volume}
    min={0}
    max={200}
    step={5}
    valueLabelCallback={(value) => `${value}%`}
  />
  <p>Emulation speed:</p>
  <MenuSlider
    bind:value={
      () => speedSliderValues.indexOf(options.speed),
      (i) => (options.speed = speedSliderValues[i])
    }
    min={0}
    max={speedSliderValues.length - 1}
    step={1}
    valueLabelCallback={(value) => `${speedSliderValues[value]}x`}
  />

  <p>Fast forward speed:</p>
  <MenuSlider
    bind:value={
      () => speedSliderValues.indexOf(options.fast_forward_speed),
      (i) => (options.fast_forward_speed = speedSliderValues[i])
    }
    min={0}
    max={speedSliderValues.length - 1}
    step={1}
    valueLabelCallback={(value) => `${speedSliderValues[value]}x`}
  />

  <button class="danger-button" onclick={() => (options = defaultOptions)}
    >Reset options</button
  >
</div>
