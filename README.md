# raylib-wasm
- Library lets you run your raylib games in your browser and on your machine with NO CHANGES in your code.

- We don't use emscripten or anything like that, just pure Rust and pure JavaScript, no dependencies (you only need to have wasm and raylib installed).

- You can see a great example of using this library here: <https://github.com/rakivo/rust-raylib-hotreload-wasm-template>. This is a template, so you can start a new repo with it.

> [!NOTE]
> Of course not all raylib functions are supported in browser at the moment, but if anyone is interested in this library, you can make a pull request, so I can see if I need to continue work on this peace of software.

# A process of porting a function from native to JS
- Go to `src/web/fns.rs`, add your desired function to the `extern` block in the `ffi` module:
```rs
extern "C" {
    // ...
    pub fn ClearBackground(color: Color);
}
```

- If the function accepts any structs, you need to pass them via their address in memory:
```rs
pub unsafe fn DrawRectangleRec(rec: Rectangle, color: Color) {
    ffi::DrawRectangleRec(std::ptr::addr_of!(rec), std::ptr::addr_of!(color));
}
```

- Then, go to `raylib.js`, find the `WebAssembly.instantiateStreaming(fetch(WASM_PATH), {...` line, and implement your function in `JS` there. For instance, this is how `DrawRectangleRec` is implemented:
```js
WebAssembly.instantiateStreaming(fetch(WASM_PATH), {
    "env": make_environment({
        // ...
        DrawRectangleRec: (rec_ptr, color_ptr) => {
            const buffer = wf.memory.buffer;
            const [x, y, w, h] = new Float32Array(buffer, rec_ptr, 4);
            const color = getColorFromMemory(buffer, color_ptr);
            ctx.fillStyle = color;
            ctx.fillRect(x, y, w, h);
        },
    })
}
```

## Inspiration

This project was inspired by [tsoding/zozlib.js](https://github.com/tsoding/zozlib.js) and [tsoding/snake-c-wasm](https://github.com/tsoding/snake-c-wasm).
