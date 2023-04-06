# Merge gamepads 🎮

## About
***Merge gamepads*** is an application which merges multiple XInput (XBOX) gamepads/controllers into one virtual gamepad.

Helpful if you want to play with somebody in a single player game with two/three game pads (you want to take turns).  
*It is especially useful, when using some remote game screen sharing software (eg. [Moonlight](https://moonlight-stream.org/) + [Sunshine](https://github.com/LizardByte/Sunshine)).*

## Usage
1. Disconnect all gamepads, cause new merged gamepad will need gamepad 0 slot to work in most games.
2. Execute `merge_gamepads.exe` from Windows Explorer or console.  
*Default gamepad refreshing rate is `10 ms`. If you want to change that, use `merge_gamepads.exe time_in_ms` instead.*
3. When prompt occurs, connect all your gamepads, which you want to merge, then press `ENTER`.
4. That's all, virtual gamepad will be present until you close application.

## Troubleshooting
**Game see only input from one, not merged gamepad**
* Try to set merged gamepad as **Preferred device** in Windows **Game controllers** settings.
* Use [HidHide tool](https://vigem.org/projects/HidHide/), to hide physical game pads in game.

**Game registers duplicate inputs**
* Use [HidHide tool](https://vigem.org/projects/HidHide/), to hide physical game pads in game.

## Build
1. Install [`rust`](https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-windows)
2. Use `cargo build` or `cargo build --release` in root directory to build binary.  
It will be placed in `target\[debug,release]\` directory.
