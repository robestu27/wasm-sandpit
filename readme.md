# Testing Rust /WASM

Can we use the strong-typing, tooling of Rust, to deploy complex logic on the client side - or logic that
may need to execute quickly?

- [x] demonstrated we can pass a tree structure from Rust/WASM (now called "Rust") - its JSON-serialized which does seem a little
wasteful, but reported as being faster than constant JS-to-WASM calls
- [ ] can we show this tree in a widget (Vue.js with PrimeFaces/PrimeVue )

## Getting it Running

To install wasm-pack:

```bash
cargo install wasm-pack
```

To build the wasm and JS:

```bash
wasm-pack build --target web
```

You need a web server to run it, simplest can be, and navigate to [localhost](http://127.0.0.1:8000/index.html)

```bash
pyhton -m http.server
```

## My Notes

- must "import" the wasm function in your JS before you can use it


## Useful Links

https://rustwasm.github.io/wasm-bindgen/introduction.html

