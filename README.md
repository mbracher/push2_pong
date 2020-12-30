# push2_pong

[![Crate](https://img.shields.io/crates/v/push2_display.svg)](https://crates.io/crates/push2_pong)

Push2Pong is the Pong game implemented in Rust for the Ableton Push2.

Ableton Push2 is a MIDI instrument with a 960x160 RGB LCD display.
Push2 is a USB composite device with a MIDI interface and a generic bulk data interface used to drive the display.

```bash
git clone https://github.com/mbracher/push2_pong
cd push2_display
cargo install push2_pong

push2_pong
```
or
```bash
git clone https://github.com/mbracher/push2_pong
cd push2_display
cargo run
```

![Photo of Pong on Push2 device](https://raw.githubusercontent.com/mbracher/push2_pong/master/push2pong.jpg)

## References
[Ableton Push Interface](https://github.com/Ableton/push-interface)

[Embedded graphics](https://github.com/embedded-graphics/embedded-graphics)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
