# Hello workd on microbit boaard

This code runs a hello world code on a microbit v2.2.0 board. This board runs on a nRF52833 chip and has a 5x5 LED matrix display.
Code's been written in rust :crab:.

## Dependencies

To run this code you'll need a couple of binary dependencies besides rust itself:

- [`cargo-embassy`](https://crates.io/crates/cargo-embassy) - A CLI to start a template for embedded programming in rust
- [`probe-rs`](https://probe.rs/) - A collection of on chip debugging tools to communicate with microchips. Install it by running the following command:

  ```bash
  curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
  ```

## Running the code

Once your code has been set, you can compile it and flash your device by running `cargo run`.
