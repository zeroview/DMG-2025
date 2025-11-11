export type ControlName = "Left" | "Right" | "Up" | "Down" | "A" | "B" | "Select" | "Start";
export type KeybindName = "Zoom in" | "Zoom out";

export class Keybind<NameType> {
  public name: NameType;
  public input: string;
  constructor(name: NameType, input: string) {
    this.name = name;
    this.input = input;
  }
}

export default class InputManager {
  public mappingToRebind: string | undefined = $state(undefined);
  private pauseCallback: () => void = () => { };
  private controlCallback: (input: ControlName, pressed: boolean) => void = () => { };
  private keybindCallback: (input: KeybindName) => void = () => { };

  public controls: Keybind<ControlName>[] = $state([
    new Keybind("Left", "ArrowLeft"),
    new Keybind("Right", "ArrowRight"),
    new Keybind("Up", "ArrowUp"),
    new Keybind("Down", "ArrowDown"),
    new Keybind("A", "x"),
    new Keybind("B", "z"),
    new Keybind("Select", "Backspace"),
    new Keybind("Start", "Enter"),
  ]);
  public keybinds: Keybind<KeybindName>[] = $state([
    new Keybind("Zoom in", "+"),
    new Keybind("Zoom out", "-"),
  ]);

  private getKey = (event: KeyboardEvent) => {
    if (event.key === " ") {
      return "Space";
    } else {
      return event.key;
    }
  }

  handleKey = (event: KeyboardEvent, pressed: boolean) => {
    let key = this.getKey(event);
    if (this.mappingToRebind) {
      if (key === "Escape") {
        this.mappingToRebind = undefined;
        return;
      }
      let keybind = this.controls.find((control) => control.name == this.mappingToRebind) as Keybind<string> | undefined;
      if (!keybind) {
        keybind = this.keybinds.find((input) => input.name == this.mappingToRebind) as Keybind<string> | undefined;
      }
      if (!keybind) {
        return;
      }
      keybind.input = key;
      this.mappingToRebind = undefined;
      return;
    }
    if (pressed && key === "Escape") {
      this.pauseCallback();
    }
    this.controls.forEach((input) => {
      if (input.input === key) {
        this.controlCallback(input.name, pressed);
      }
    });
    if (pressed) {
      this.keybinds.forEach((keybind) => {
        if (keybind.input === key) {
          this.keybindCallback(keybind.name);
        }
      });
    }
  }

  onPause(callback: () => void) {
    this.pauseCallback = callback;
  }

  onControlInput(callback: (input: ControlName, pressed: boolean) => void) {
    this.controlCallback = callback;
  }

  onKeybindPressed(callback: (keybind: KeybindName) => void) {
    this.keybindCallback = callback;
  }
}
