<div align="center">
  <h1>Conways Game of Life</h1>
  <h2>in Rust Webassembly</h2>
</div>

# Tutorial

We're broadly following the [Rust WASM tutorial](https://rustwasm.github.io/book/game-of-life/introduction.html)

## Setup and Installation

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
