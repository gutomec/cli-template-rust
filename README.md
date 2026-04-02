# cli-template-rust

Production-grade CLI template using Clap and Cargo.

## Features

- **Clap Framework**: Derive macro for powerful, ergonomic CLI argument parsing
- **Type Safety**: Rust's type system prevents entire classes of bugs
- **Performance**: Compiled binaries are fast and lightweight
- **Testing**: Integration and unit testing infrastructure
- **Benchmarking**: Built-in performance benchmarking
- **Code Quality**: clippy for linting with zero warnings
- **CI/CD**: GitHub Actions for multi-platform testing
- **Release**: Cross-compilation for multiple architectures
- **Documentation**: Comprehensive guides and examples

## Quick Start

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- Cargo (comes with Rust)

### Build and Run

```bash
# Build in debug mode (fast compilation)
cargo build

# Run the CLI
./target/debug/cli-template hello

# Run tests
cargo test

# Build optimized release
cargo build --release

# Run release binary
./target/release/cli-template hello
```

## Project Structure

```
cli-template-rust/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library code
│   ├── commands/
│   │   ├── mod.rs           # Command modules
│   │   ├── hello.rs         # Hello command
│   │   └── config.rs        # Config command
│   └── utils/
│       ├── mod.rs
│       └── helpers.rs       # Helper functions
├── tests/
│   ├── integration_test.rs  # Integration tests
│   └── cli_test.rs          # CLI tests
├── benches/
│   └── bench.rs             # Performance benchmarks
├── .github/
│   └── workflows/           # GitHub Actions CI/CD
│       ├── test.yml         # Test workflow
│       ├── lint.yml         # Linting workflow
│       └── release.yml      # Release workflow
├── Cargo.toml               # Package manifest
├── Cargo.lock               # Locked dependencies
└── README.md                # This file
```

## Available Commands

### `hello`

Simple greeting command.

```bash
cargo run -- hello
# Output: Hello, World!

cargo run -- hello --name Alice
# Output: Hello, Alice!
```

### Development Workflows

| Command | Purpose |
|---------|---------|
| `cargo build` | Debug build (fast compile) |
| `cargo build --release` | Release build (optimized) |
| `cargo test` | Run all tests |
| `cargo test -- --nocapture` | Show test output |
| `cargo clippy` | Lint code (zero warnings) |
| `cargo fmt` | Format code |
| `cargo doc --open` | Generate and view docs |
| `cargo bench` | Run benchmarks |

## Testing

Tests include both unit tests and integration tests.

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_hello

# Run integration tests only
cargo test --test integration_test

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Test Structure

- Unit tests in `src/lib.rs` and modules
- Integration tests in `tests/` directory
- Test utilities and fixtures in helper modules

## CI/CD Workflows

### `test.yml`
- Runs on: Windows, macOS, Linux
- Rust versions: 1.70.0, stable, nightly
- All tests must pass

### `lint.yml`
- clippy with no warnings allowed
- cargo fmt check (formatting)
- Runs on Ubuntu (fastest)

### `release.yml`
- Triggered on version tags (v*.*.*)
- Builds for multiple platforms/architectures
- Creates release artifacts

## Cross-Platform Support

The template supports multiple OS/architecture combinations:

```
- Linux x86_64 (x86_64-unknown-linux-gnu)
- macOS Intel (x86_64-apple-darwin)
- macOS ARM64 (aarch64-apple-darwin)
- Windows x86_64 (x86_64-pc-windows-msvc)
```

Cross-compile for specific target:

```bash
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc

# View all available targets
rustup target list
```

## Performance

### Optimization Levels

Debug build (development):
```bash
cargo build
# Fast compilation, slower runtime
```

Release build (production):
```bash
cargo build --release
# Slower compilation, fast runtime
# See Cargo.toml [profile.release] for LTO settings
```

### Benchmarking

Run benchmarks:
```bash
cargo bench
```

The template includes benchmark configuration in `Cargo.toml`:
```toml
[profile.bench]
opt-level = 3
lto = true
```

## Publishing to crates.io

### One-time Setup

1. Create account at https://crates.io
2. Generate API token on crates.io
3. Configure Cargo: `cargo login`
4. Verify Cargo.toml is correct

### Publishing

```bash
# Check before publishing
cargo publish --dry-run

# Tag the release
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0

# GitHub Actions will publish to crates.io automatically
# Or publish manually:
cargo publish
```

## Using Clap Derive Macros

The template uses Clap's derive macros for clean CLI definition:

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Command to execute
    #[arg(short, long)]
    command: String,

    /// Optional argument
    #[arg(short, long, default_value = "default")]
    option: String,
}

fn main() {
    let args = Args::parse();
    println!("Command: {}", args.command);
    println!("Option: {}", args.option);
}
```

## Extending the Template

### Adding New Commands

1. Create new module in `src/commands/`
2. Define command logic
3. Register in `src/main.rs`
4. Write integration tests

Example:

```rust
// src/commands/goodbye.rs
pub fn goodbye(name: &str) -> String {
    format!("Goodbye, {}!", name)
}

// Add to src/main.rs
use commands::goodbye;
// ...
Commands::Goodbye { name } => {
    println!("{}", goodbye(&name));
}
```

### Adding Dependencies

```bash
# Add a dependency
cargo add serde --features derive

# Add a dev dependency
cargo add --dev assert_cmd predicates
```

## Troubleshooting

### Build errors

- Check Rust version: `rustc --version` (should be 1.70+)
- Update Rust: `rustup update`
- Clean build: `cargo clean`

### Clippy warnings

- Address warnings: `cargo clippy --fix`
- Or suppress with `#[allow(clippy::warning_name)]`
- Never ship with clippy warnings

### Test failures

- Run tests with output: `cargo test -- --nocapture`
- Check test output for assertions
- Verify dependencies are correct

### Cross-compilation issues

- Install target: `rustup target add <target>`
- Verify Cargo.lock doesn't have platform-specific deps
- Check for conditional compilation (`#[cfg(...)]`)

## Best Practices

- Use meaningful error types with anyhow/thiserror
- Write comprehensive tests (aim for >80% coverage)
- Document public APIs with doc comments
- Use `#[must_use]` for important functions
- Keep main.rs simple, move logic to lib.rs
- Use clippy to catch common mistakes
- Profile before optimizing
- Handle errors gracefully
- Log important operations

## Performance Tips

1. Use `cargo build --release` for accurate performance tests
2. Profile with `cargo flamegraph` (requires flamegraph tool)
3. Benchmark with `cargo bench`
4. Use release builds with LTO enabled (see Cargo.toml)
5. Minimize allocations in hot paths

## Dependencies

### Core
- **clap** - CLI argument parsing (with derive feature)
- **anyhow** - Error handling

### Testing
- **assert_cmd** - Command integration tests
- **predicates** - Assertions for test output

### Benchmarking
- Built-in criterion crate (optional)

## Minimum Supported Rust Version (MSRV)

```toml
rust-version = "1.70"
```

The template specifies MSRV to support older Rust installations.

## License

MIT - See LICENSE file for details

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Run linting: `cargo clippy -- -D warnings`
6. Format code: `cargo fmt`
7. Submit a pull request

## Resources

- [Clap Documentation](https://docs.rs/clap/latest/clap/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust CLI Book](https://rust-cli.github.io/book/)
- [Cargo Guide](https://doc.rust-lang.org/cargo/)
