![DMG-2025](/app/assets/logo_large.png)

## Features
- Modern and responsive web UI with pixelated aesthetic
  - Written with raw CSS
- Reasonably accurate DMG (first Game Boy edition) emulation
  - Core written in _blazingly fast_ Rust, compiled to WebAssembly
  - Plays most games as on real hardware  
  - Passes [`cpu_instrs`](https://github.com/retrio/gb-test-roms/tree/master/cpu_instrs), [`dmg-acid2`](https://github.com/mattcurrie/dmg-acid2) etc.
- GPU-based display rendering
  - Customizable color palettes
  - Configurable shader effects, running on WebGL
- Integrated browser for ROMs from [Homebrew Hub](https://hh.gbdev.io/)
  - Run over 800 community-made games and demos with just one click
- Save states
- Automatic saving of games (that have support for it)
- Fast-forward / slow-motion
- Input rebinding

## Local installation
- Install dependencies:
  - [`npm`](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) (or another package manager of your choice)
  - [`rustc`](https://rust-lang.org/tools/install/)  and [`cargo`](https://rust-lang.org/tools/install/)
  - [`wasm-pack`](https://drager.github.io/wasm-pack/installer/)
- Run dev server with `npm run dev`
- Create production build with `npm run build`
