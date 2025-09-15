# `tomu_usb_simple_client` &emsp; [![Latest Version]][crates.io] [![Docs version]][docs.rs]

A Rust client for [Tomu's][tomu] [`usb_simple` sample app][usb-simple], which activates the
[Tomu's][tomu] LEDs in response to simple USB control commands.

## Running the demo

1. Compile and install [the `usb_simple` app][usb-simple] on your Tomu with auto-run enabled.

1. _On Windows only:_ [assign the `WinUsb Device` driver to the Tomu][windows-driver].

1. [Install a recent Rust toolchain][rustup].

1. Run the demo:

   ```sh
   cargo run --example demo
   ```

[crates.io]: https://crates.io/crates/tomu_usb_simple_client
[Docs version]: https://img.shields.io/docsrs/tomu_usb_simple_client.svg
[docs.rs]: https://docs.rs/tomu_usb_simple_client/
[Latest Version]: https://img.shields.io/crates/v/tomu_usb_simple_client.svg
[rustup]: https://rustup.rs/
[tomu]: https://tomu.im/
[usb-simple]: https://github.com/im-tomu/tomu-samples/tree/master/usb_simple
[windows-driver]: https://github.com/im-tomu/tomu-samples/blob/master/usb_simple/README.md#windows-driver
