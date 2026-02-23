# Rosin Quickstart

This repo is a Rust project that is pre-configured to use the `hot-reload` feature of the [Rosin GUI Library](https://github.com/sailbrush/rosin).

Hot-reload is an experimental feature that swaps out the code of an application while it's running so developers can see what their code does as they write it. It's intended for use during development only, is likely unsound, and may give the appearance of bugs where there are none.

When compiling in release mode, the `hot-reload` feature flag does nothing.

## Prerequisites

- The latest stable Rust toolchain must be installed.
    - To install it, follow the [official installation guide](https://rust-lang.org/learn/get-started/).
- `cargo-watch` must be installed for the `watch.sh` script to work.
    - To install it, run: `cargo install cargo-watch`

## Instructions

1) Clone this repo by running `git clone https://github.com/sailbrush/rosin-quickstart.git`
    - You may want to rename the directory, and replace this readme with your own.
    - If you change the package name in `Cargo.toml`, make sure to change the bin name to match.
2) Run `cargo run &` to run the program in debug mode as a background job.
    - This will return you to the prompt immediately so you can run other commands.
    - You'll still see cargo's output, but that won't stop you from running the next command.
3) When the application opens, run `./watch.sh` to automatically re-compile when source code is modified.

That's it!

Now, when you make changes to `lib.rs`, the new code will automatically be loaded into the running application without losing state. When the State struct is modified, the app will restart and load data from the previous instance on a best effort basis.

To stop the watch script, press `ctrl + C` in your terminal window.

## Details

In order for `hot-reload` to work, the following conditions must be met:

- The `hot-reload` feature flag must be enabled on the `rosin` crate.
- The app must be compiled in debug mode.
- The crate must be configured to compile as both a binary and a dylib.
- The binary name must match the package name.
- All view/wgpu callbacks must be annotated with `#[unsafe(no_mangle)]`.
- The app state must implement `Serialize`, `Deserialize`, and `TypeHash`.
- The app state must derive `Expose`.
