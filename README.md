# cli-template-rust

Production CLI template using Clap and Cargo.

## Quick Start

```bash
cargo build --release
./target/release/cli-template hello
cargo test
```

## Commands

- `hello` - Example hello command

## Development

```bash
cargo build              # Debug build
cargo build --release   # Release build (optimized)
cargo test              # Run tests
cargo clippy            # Lint code
cargo fmt               # Format code
cargo bench             # Run benchmarks
```

## Performance

Compiled in release mode with LTO enabled for optimal performance.

```bash
cargo build --release
```

## Publishing

```bash
cargo publish
```

## License

MIT
