A simple Rust-based prototype for the Jungle board game

## Quick start

Clone this repository and simply run:

```bash
cargo run
```

This will take quite some time to compile (mainly the libraries), then it should run smoothly.

If this runs too slowly (in runtime, not compile time) for some reason, try adding the `--release` flag.

## If Rust is not installed yet

Install Rust (preferably with `rustup`) if you haven't already:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

(Or, even better, use your OS's package manager, e.g., `yay` or `homebrew`)

Then, install the stable toolchain (esp. if you installed with a package manager):

```bash
Rustup toolchain install stable
```

## Get lines of code

```bash
find --file rs | xargs cat | wc -l
```
