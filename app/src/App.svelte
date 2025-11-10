<script lang="ts">
  import Browser from "./Browser.svelte";
  import MenuSlider from "./MenuSlider.svelte";
  import EmulatorManager from "./manager.svelte";
  import { Color, Palette } from "DMG-2025";
  import { fade, fly } from "svelte/transition";
  import { onMount } from "svelte";

  let manager = new EmulatorManager();
  let currentPage = $state(0);

  let transitionsEnabled = $state(true);
  let transitionDuration = 300;
  let transitionLength = 200;
  let lastPage = 0;
  function getTransition() {
    if (!transitionsEnabled) {
      return { y: 0, duration: 0 };
    }
    let sign = currentPage > lastPage ? 1 : -1;
    lastPage = currentPage;
    return { y: sign * transitionLength, duration: transitionDuration };
  }

  class Keybind {
    public name: string;
    public input: string;
    constructor(name: string, input: string) {
      this.name = name;
      this.input = input;
    }
  }

  const inputMap = $state([
    new Keybind("Left", "ArrowLeft"),
    new Keybind("Right", "ArrowRight"),
    new Keybind("Up", "ArrowUp"),
    new Keybind("Down", "ArrowDown"),
    new Keybind("A", "x"),
    new Keybind("B", "z"),
    new Keybind("Select", "Backspace"),
    new Keybind("Start", "Enter"),
  ]);
  const keybindMap = $state([
    new Keybind("Zoom in", "+"),
    new Keybind("Zoom out", "-"),
  ]);
  function getKey(event: KeyboardEvent) {
    if (event.key === " ") {
      return "Space";
    } else {
      return event.key;
    }
  }
  function handleKey(event: KeyboardEvent, pressed: boolean) {
    if (rebindableInput) {
      if (getKey(event) === "Escape") {
        rebindableInput = undefined;
        return;
      }
      let keybind = inputMap.find((input) => input.name == rebindableInput);
      if (!keybind) {
        keybind = keybindMap.find((input) => input.name == rebindableInput);
      }
      if (!keybind) {
        return;
      }
      keybind.input = getKey(event);
      rebindableInput = undefined;
    }
    if (pressed && getKey(event) === "Escape") {
      if (!manager.initialized) {
        return;
      }
      manager.toggle_execution();
      if (!manager.running) {
        currentPage = 0;
      }
    }
    inputMap.forEach((input) => {
      if (input.input === getKey(event)) {
        manager.updateInput(input.name, pressed);
      }
    });
    if (pressed) {
      keybindMap.forEach((keybind) => {
        if (keybind.input === getKey(event)) {
          switch (keybind.name) {
            case "Zoom in":
              scaleSliderVal.value = Math.min(scaleSliderVal.value + 1, 5);
              manager.options.scale = scaleSliderVal.value;
              manager.updateOptions();
              break;
            case "Zoom out":
              scaleSliderVal.value = Math.max(scaleSliderVal.value - 1, -5);
              manager.options.scale = scaleSliderVal.value;
              manager.updateOptions();
              break;
          }
        }
      });
    }
  }

  let rebindableInput: string | undefined = $state(undefined);

  const palettes: Record<string, Palette> = {
    LCD: new Palette(
      new Color(0.327778, 0.5028864, 0.0047769),
      new Color(0.2581828, 0.4125426, 0.0047769),
      new Color(0.0295568, 0.1221387, 0.0295568),
      new Color(0.0047769, 0.0395462, 0.0047769),
    ),
    Clear: new Palette(
      new Color(0.7454042, 0.9386857, 0.6307571),
      new Color(0.2462013, 0.5271151, 0.1620293),
      new Color(0.0343398, 0.1384316, 0.0930589),
      new Color(0.0024282, 0.009134, 0.0144438),
    ),
    Raw: new Palette(
      new Color(1.0, 1.0, 1.0),
      new Color(0.6666, 0.6666, 0.6666),
      new Color(0.3333, 0.3333, 0.3333),
      new Color(0.0, 0.0, 0.0),
    ),
  };
  let defaultPalette = Object.keys(palettes)[0];
  let currentPalette = $state(defaultPalette);
  manager.options.update_palette(palettes[defaultPalette]);

  function swapPalette() {
    let paletteNames = Object.keys(palettes);
    let paletteIndex = paletteNames.indexOf(currentPalette);
    paletteIndex++;
    if (paletteIndex >= paletteNames.length) {
      paletteIndex = 0;
    }
    currentPalette = paletteNames[paletteIndex];

    manager.options.update_palette(palettes[currentPalette]);
    manager.updateOptions();
  }

  const speedSliderValues = [
    0.01, 0.05, 0.1, 0.3, 0.5, 0.7, 0.8, 0.9, 1, 1.1, 1.3, 1.5, 2, 3, 5, 10, 20,
  ];
  let speedSliderVal = $state({
    value: speedSliderValues.indexOf(1),
    effect: (value: number) => {
      manager.options.speed = speedSliderValues[value];
      manager.updateOptions();
    },
  });

  let scaleSliderVal = $state({
    value: 0,
    effect: (val: number) => {
      manager.options.scale = val;
      manager.updateOptions();
    },
  });

  let volumeSliderVal = $state({
    value: 100,
    effect: (value: number) => {
      manager.options.volume = value / 100;
      manager.updateOptions();
    },
  });

  let backgroundGlowStrengthSliderVal = $state({
    value: 60,
    effect: (value: number) => {
      manager.options.background_glow_strength = value / 100;
      manager.updateOptions();
    },
  });

  let displayGlowStrengthSliderVal = $state({
    value: 30,
    effect: (value: number) => {
      manager.options.display_glow_strength = value / 100;
      manager.updateOptions();
    },
  });

  let glowQualitySliderVal = $state({
    value: 5,
    effect: (value: number) => {
      manager.options.glow_iterations = value * 2;
      manager.updateOptions();
    },
  });

  let glowRadiusSliderVal = $state({
    value: 0.5,
    effect: (value: number) => {
      manager.options.glow_radius = value;
      manager.updateOptions();
    },
  });

  let ambientLightSliderVal = $state({
    value: 0.3,
    effect: (value: number) => {
      manager.options.ambient_light = value;
      manager.updateOptions();
    },
  });

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
      let isZip = zipMimeTypes.includes(file.type);
      files[0].arrayBuffer().then((rom) => manager.loadRom(rom, isZip));
    }
  });

  let popupVisible = $state(false);
  let popupText = $state("");
  function showMessage(msg: string) {
    popupText = msg;
    popupVisible = true;
    setTimeout(() => {
      popupVisible = false;
    }, 2000);
  }

  let firstLoad = true;
  let eventListener: HTMLElement;
  onMount(() => {
    if (eventListener) {
      eventListener.addEventListener("romloaded", (e) => {
        const event = e as CustomEvent;
        document.title = `${event.detail} - DMG-2025`;
        console.info("Successfully loaded ROM");
        if (firstLoad) {
          showMessage("Press Esc to return to menu");
          firstLoad = false;
        }
        manager.toggle_execution();
      });
      eventListener.addEventListener("romloadfailed", (e) => {
        const event = e as CustomEvent;
        let msg = `Failed to load ROM: ${event.detail}`;
        console.error(msg);
        showMessage(msg);
      });
    }
  });
</script>

<svelte:window
  on:keydown={(event) => handleKey(event, true)}
  on:keyup={(event) => handleKey(event, false)}
/>

<main>
  <p id="eventListener" bind:this={eventListener}></p>
  {#if popupVisible}
    <p
      class="popup"
      in:fly={{ y: 200, duration: 600 }}
      out:fade={{ duration: 2000 }}
    >
      {popupText}
    </p>
  {/if}
  <canvas id="canvas" tabindex="-1"></canvas>
  {#if !manager.running}
    <div
      class="menu"
      transition:fade={{ duration: transitionsEnabled ? 100 : 0 }}
    >
      <div class="menu-sidebar">
        <button onclick={() => (currentPage = 0)}>MAIN</button>
        <button onclick={() => (currentPage = 1)}>BROWSER</button>
        <button onclick={() => (currentPage = 2)}>VISUALS</button>
        <button onclick={() => (currentPage = 3)}>INPUT</button>
      </div>
      {#if currentPage == 1}
        <div class="menu-container" in:fly={getTransition()}>
          <Browser {manager} />
        </div>
      {:else if currentPage == 2}
        <div class="menu-container" in:fly={getTransition()}>
          <div class="menu-grid">
            <h3>General</h3>
            <p>Scaling offset:</p>
            <MenuSlider value={scaleSliderVal} min={-5} max={5} step={1} />
            <p>Color palette:</p>
            <button onclick={swapPalette}>{currentPalette}</button>
            <p>UI transitions:</p>
            <button onclick={() => (transitionsEnabled = !transitionsEnabled)}>
              {transitionsEnabled ? "On" : "Off"}
            </button>

            <h3>Glow</h3>
            <p>BG strength:</p>
            <MenuSlider
              value={backgroundGlowStrengthSliderVal}
              min={0}
              max={100}
              step={1}
              valueLabelCallback={(value) => `${value}%`}
            />
            <p>Display strength:</p>
            <MenuSlider
              value={displayGlowStrengthSliderVal}
              min={0}
              max={100}
              step={1}
              valueLabelCallback={(value) => `${value}%`}
            />
            <p>Quality:</p>
            <MenuSlider
              value={glowQualitySliderVal}
              min={0}
              max={10}
              step={1}
            />
            <p>Radius:</p>
            <MenuSlider
              value={glowRadiusSliderVal}
              min={0}
              max={5}
              step={0.1}
            />
            <p>Ambient light:</p>
            <MenuSlider
              value={ambientLightSliderVal}
              min={0}
              max={1}
              step={0.01}
            />
          </div>
        </div>
      {:else if currentPage == 3}
        <div class="menu-container" in:fly={getTransition()}>
          <div class="menu-grid" style="grid-template-columns: 10rem 15rem">
            <h3>Controls</h3>
            {#each inputMap as input (input.name)}
              <p>{input.name}</p>
              {#if rebindableInput == input.name}
                <button style="color:grey">[ Rebinding... ]</button>
              {:else}
                <button onclick={() => (rebindableInput = input.name)}
                  >{input.input}</button
                >
              {/if}
            {/each}

            <h3>Keybinds</h3>
            {#each keybindMap as input (input.name)}
              <p>{input.name}</p>
              {#if rebindableInput == input.name}
                <button style="color:grey">[ Rebinding... ]</button>
              {:else}
                <button onclick={() => (rebindableInput = input.name)}
                  >{input.input}</button
                >
              {/if}
            {/each}
          </div>
        </div>
      {:else}
        <div class="menu-container" in:fly={getTransition()}>
          <input
            id="fileInput"
            accept=".gb,.zip"
            type="file"
            bind:files
            style="display: none"
          />
          <div class="button-row">
            <button
              onclick={() => document.getElementById("fileInput")?.click()}
            >
              Load ROM
            </button>
            <button onclick={() => (currentPage = 1)}
              >Browse Homebrew Hub</button
            >
          </div>
          <p style="height: 50px"></p>

          <div class="menu-grid">
            <p>Volume:</p>
            <MenuSlider
              value={volumeSliderVal}
              min={0}
              max={200}
              step={1}
              valueLabelCallback={(value) => `${value}%`}
            />
            <p>Emulation speed:</p>
            <MenuSlider
              value={speedSliderVal}
              min={0}
              max={speedSliderValues.length - 1}
              step={1}
              valueLabelCallback={(value) => `${speedSliderValues[value]}x`}
            />
          </div>
        </div>
      {/if}
    </div>
  {/if}
</main>
