# penGui

An immediate-mode library written in Rust

To install Rust and everything needed, and run the example :

- Install rust. Instructions can be found [here](https://www.rust-lang.org/tools/install))
- If rust is already installed, make sure that your toolchain version is >=1.47.0
- Clone this repository
- To run the example, use the command `cargo run --example glium-experimental`
- Eat some cookies while it compiles (it might take two or three minutes to gather all packages necessary and to compile everything)


Inside the example:
- move the mouse while `alt` is pressed to move the camera around, use the mouse wheel to zoom in and out. Clicks are not yet implemented.
- use `alt+D` to select the display mode: fill, lines or points. The first gives the normal result, the two others are used to debug the geometry.
- press `ctrl+w` to quit the application.
- The two rectangles at the middle are buttons, but they are not clickable yet. Buttons support colors and textures.

## Tests

To run the test suite, run `cargo test`.

If you want to get a coverage report on the test suite, run the following commands :

```sh
cargo install cargo-cov
rustup install nightly
cargo +nightly cov test
cargo +nightly cov report --open
```