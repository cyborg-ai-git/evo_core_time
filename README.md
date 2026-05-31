# evo_core_time

Time utilities for the Evo Rust workspace.

`evo_core_time` is the facade crate. It re-exports the platform implementation from `evo_core_time_rust`, giving downstream crates one dependency for native Rust and `wasm32` builds.

## Crates

| Path | Crate | Purpose |
| --- | --- | --- |
| `evo_core_time` | `evo_core_time` | Public facade crate with target-based re-exports. |
| `rust/evo_core_time_rust` | `evo_core_time_rust` | Time implementation for native Rust and WebAssembly targets. |

## Features

- `UTime::time_ns()` returns the current time in nanoseconds since the Unix epoch.
- `UTime::time_ms()` returns the current time in milliseconds since the Unix epoch.
- `UTime::time_s()` returns the current time in seconds since the Unix epoch.
- `UTimeElapsed::ns(start_ns)` returns elapsed nanoseconds from a nanosecond timestamp.
- `UTimeElapsed::ms(start_ns)` returns elapsed milliseconds as `f64`.
- `UTimeElapsed::s(start_ns)` returns elapsed seconds as `f64`.
- `UTimeExt::to_local_time(time_ns, format)` formats a nanosecond timestamp as local time on native targets.

On native targets, time is read from `std::time::SystemTime`. On `wasm32`, time is read from `js_sys::Date`.

## Installation

For another workspace crate, depend on the facade crate:

```toml
[dependencies]
evo_core_time = { path = "../evo_core_time/evo_core_time" }
```

For direct implementation-crate use inside this workspace:

```toml
[dependencies]
evo_core_time_rust = { path = "../evo_core_time/rust/evo_core_time_rust" }
```

## Usage

Read the current time:

```rust
use evo_core_time::UTime;

fn main() {
    let now_ns = UTime::time_ns();
    let now_ms = UTime::time_ms();
    let now_s = UTime::time_s();

    println!("ns={now_ns} ms={now_ms} s={now_s}");
}
```

Measure elapsed time:

```rust
use evo_core_time::{UTime, UTimeElapsed};

fn main() {
    let start = UTime::time_ns();

    // Work to measure.

    let elapsed_ns = UTimeElapsed::ns(start);
    let elapsed_ms = UTimeElapsed::ms(start);
    let elapsed_s = UTimeElapsed::s(start);

    println!("elapsed: {elapsed_ns} ns / {elapsed_ms} ms / {elapsed_s} s");
}
```

Format native local time:

```rust
use evo_core_time::{UTime, UTimeExt};

fn main() {
    let formatted = UTimeExt::to_local_time(UTime::time_ns(), Some("%Y-%m-%d %H:%M:%S"));
    println!("{formatted}");
}
```

If no format is provided, `UTimeExt::to_local_time` uses `%Y::%m::%d %H:%M:%S`.

## Development

Run checks from the repository root:

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features
```

Helper scripts live under `scripts/`:

```bash
scripts/run_cargo_clean.sh
scripts/run_cargo_update.sh
scripts/run_documentation.sh
scripts/run_git_push.sh
scripts/run_github_repository.sh
```

## License

CC BY-NC-ND 4.0 Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International.

## Links

- GitHub: <https://github.com/cyborg-ai-git/evo_core_time>
- CyborgAI: <https://cyborgai.fly.dev>
