# Testing Rust /WASM

Can we use the strong-typing, tooling of Rust, to deploy complex logic on the client side - or logic that
may need to execute quickly?

- [x] demonstrated we can pass a tree structure from Rust/WASM (now called "Rust") - its JSON-serialized which does seem a little
wasteful, but reported as being faster than constant JS-to-WASM calls
- [x] can we show this tree in a widget (Vue.js with PrimeFaces/PrimeVue )
- [x] can we show a tree table in a widget (Vue.js with PrimeFaces/PrimeVue )
- [ ] load data from a backend service 

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
- what is the correct way to initialize the wasm module .. synch or async .. dealing with the promise

### PrimeVue TreeTable

Data shown in each row must be in a object called `data` else not shown in tree


## Useful Links

https://rustwasm.github.io/wasm-bindgen/introduction.html

