<div align="center">
  <h1>Conway's Game of Life</h1>
  <h2>in Rust Webassembly</h2>
</div>

# Tutorial

We're broadly following the [Rust WASM tutorial](https://rustwasm.github.io/book/game-of-life/introduction.html). If you're here and keen to learn how to use Rust for WebAssembly, go through that tutorial. That's how I got started with this code, and I highly recommend it. It's easier than expected. It's also clear that folks have put plenty of work into making it fairly easy to integrate with WASM.

This code has evolved a bit beyond the tutorial in a few ways:
- prettier in a retro-ish way that I enjoy
- performance improvements
- implemented draw routines in both Rust and JS the can be compared

## Setup and Installation

- [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already done so
- `npm` - install as per usual
- `wasm-pack` - install using `cargo install wasm-pack`
- `npm` dependencies - run `npm install` in the `./www` directory

## Build, test, run

```
./build
./test
./run
```

# Notes

These are useful tools too:

- [WABT project](https://github.com/WebAssembly/wabt), installed via `apt`, including `wasm-objdump` and `wasm2wat`
- [wasm-nm](https://github.com/fitzgen/wasm-nm), installed via `cargo install wasm-nm` for compiled object names (exports, etc.)
- [WASM plugin](https://marketplace.visualstudio.com/items?itemName=dtsvet.vscode-wasm) for VS Code (allows right-click disassembly)

# Template used 

I've used this template to bootstrap this project: [`wasm-pack-template`](https://github.com/rustwasm/wasm-pack-template)
