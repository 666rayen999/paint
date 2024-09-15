# Paynt - Web Paint App

This is a simple paint application built using the [macroquad](https://macroquad.rs/) game framework in Rust, and compiled to WebAssembly (Wasm) for the web. The app allows users to draw shapes like lines, rectangles, and ellipses directly in the browser.

### Features

- **Draw Freehand**: Hold down the mouse button to draw freely on the canvas.
- **Draw Shapes**: You can draw lines, rectangles, and ellipses.
- **Clear Canvas**: Clear the canvas by pressing the `C` key.
- **Responsive Canvas**: Works directly in the browser with full responsiveness.

### Controls

- Press `L` to switch to **Line** drawing mode.
- Press `R` to switch to **Rectangle** drawing mode.
- Press `E` to switch to **Ellipse** drawing mode.
- Press `P` to switch to **Freehand** drawing mode.
- Press `C` to **Clear** the canvas.

### Usage

You can open it here: [Paynt](https://paynt.netlify.app/)

Or download the source code and compile it

```sh
git clone https://github.com/666rayen999/paint
cd paint
cargo r -r
```
