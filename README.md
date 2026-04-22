# `zsh-src`

This crate contains the logic for resolving the source code of zsh, which is used by the `zsh-ffi` crate to build against the correct version of zsh. Currently, it does this by downloading the source code from the official zsh repository on GitHub, but in the future, it may also support other sources, such as a local copy of the source code like provided through [`zsh-dev`](https://packages.debian.org/sid/zsh-dev) on Debian-based systems.


## Prerequisites
* `zsh` (the zsh binary must be available in the system's PATH)
* `make`


## Quick Start
Add `zsh-src` as a dependency in your `Cargo.toml`:
```sh
cargo add zsh-src
```

## Usage

Use the `ZshSource` struct to resolve the source code of zsh and get the path to the source code. This is typically used in a `build.rs` script to tell Cargo to rerun the build script if the source code changes.

```rust
use zsh_src::ZshSource;
fn main() -> io::Result<()> {
    let ZshSource { source, .. } = resolve()?;
    println!("cargo::rerun-if-changed={}", source.display());
    Ok(())
}
```

<!-- TODO: Optionally support using development headers like `zsh-dev` [on Debian](https://packages.debian.org/trixie/zsh-dev) -->
