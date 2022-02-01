References: 
- https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Basic_2D_animation_example for basic 2D animation.
- https://rustwasm.github.io/wasm-bindgen/examples/webgl.html for Rust bindings to webgl (if applicable)
    - might make more sense using this instead of trying to do the glue in JS itself

Todo: 
- figure out how to pass arrays between Rust and WebGL
- working assumption: single large rectangle, and use a shader to draw block colours
- convert the whole project to TS instead
- also try out writing the original rendering step in Rust (without WebGL) and benchmark