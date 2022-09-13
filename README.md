_A simple Rust-based prototype for the Jungle board game_

## Quick start

If you haven't installed Rust, see below for a simple guide.

Clone this repository and run the application:

```bash
git clone https://github.com/mhho3082/jungle-rs --depth 1

cd jungle-rs

cargo run
```

This will take quite some time to compile (mainly the libraries), then it should run smoothly.

If this runs too slowly (in runtime, not compile time) for some reason, try adding the `--release` flag.

Add `--` before this application's flags to use them:

```bash
cargo run -- -d # debug mode
```

So, to get the list of flags:

```bash
cargo run -- -h
```

## Compile

Just run the following line:

```bash
cargo build --release
```

## How to install Rust

Installing stable Rust through `rustup` is recommended:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Or, even better, use your OS's package manager, e.g., `yay` or `homebrew`,
to install `rustup`, then install the stable toolchain:

```bash
Rustup toolchain install stable
```

## Get lines of code

```bash
find --file rs | xargs cat | wc -l
```

If you want to learn Rust (say, for fun, like me),
I would like to refer you to
[the Rust Book](https://doc.rust-lang.org/book/),
which I have found really helpful.

## What could be done better

**The UI**
The current UI is as crude as it can be.
It cannot do `getch()`-style inputs,
which require no <kbd>Enter</kbd> presses;
nor does it handle mouse presses at all.
While I did not put my efforts on this as I worry about
a large library size (I cannot effortlessly implement
my own cross-platform `getch()`, can I?),
I do wonder if Java provides a better opportunity
to cross-platform CLI/TUI with its JVM.

**The AI**
I just added a simple random AI to prove my concept here.
We can add a naive version, but how to nicely implement one
(nor its algorithm) came to my mind these few days.
I will have to think about it for longer, I guess.
Any suggestions will be greatly appreciated.

**Time Machine**
I... just could not find anywhere to implement
a time machine in the current UI.
The architecture for time machines are all provided
in the model; someone just needs to hook to the
`model.current` value to scaffold a time machine.
(Yup, just change that value and refresh, and it will all work.)
If we don't really need a time machine,
then we can instead simplify `model.current` away...
or not. Not having to type `model.history.last().unwrap()`
every time is quite nice.

**Unit tests**
I mean, this is just a prototype.
All things _should work_, per se,
but I cannot guarantee them; feel free to
play around and find bugs before we port this
code to Java. If you want to try writing tests yourselves,
Rust have in-built testing support;
see [the Rust Book](https://doc.rust-lang.org/book/ch11-00-testing.html).
