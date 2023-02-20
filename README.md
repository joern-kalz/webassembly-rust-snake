# webassembly-rust-snake

## Run

### Prerequisites

[Install Rust](https://www.rust-lang.org/tools/install)

Install wasm-pack and devserver

```bash
cargo install wasm-pack devserver
```

### Build

Build with

```bash
wasm-pack build --target web
```

### Serve

```bash
devserver
```

Open [http://localhost:8080/](http://localhost:8080/) in your browser
