# `Rust ADS1256 Driver`

> A platform agnostic driver to interface with the  [ADS1256][] (analog-digital converter),
 based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.

## The Device

The ADS1256 are extremely low-noise, 24-bit analog-to-digital (A/D) converters.
They provide complete high-resolution measurement solutions for the most demanding applications.

Details and datasheet: http://www.ti.com/product/ADS1256

##  Status

- [x] Support one-shot  measurement
- [x] Configurable sample rate /gain
- [ ] Support continuous  measurement


## Examples

You can find example of communication from Raspberry PI with  ADS1256 board under 
[`examples/linux_raspi.rs`](examples/linux_raspi.rs)

To build the Raspberry PI example run :
cargo build --examples

The High-Precision AD/DA board was used for testing.
[AD/DA board] https://www.waveshare.com/wiki/High-Precision_AD/DA_Board

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

