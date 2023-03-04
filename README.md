# webassembly-rust-snake

## Develop

### Prerequisites

[Install Rust](https://www.rust-lang.org/tools/install)

Install wasm-pack, devserver and cargo-watch

```bash
cargo install wasm-pack devserver cargo-watch
```

### Build interactively

Build interactively on each change

```bash
cargo watch -- wasm-pack build --target web
```

### Serve

```bash
devserver
```

Open [http://localhost:8080/](http://localhost:8080/) in your browser

## Build

Build with

```bash
wasm-pack build --target web
```
