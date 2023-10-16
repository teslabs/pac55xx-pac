# pac55xx-pac

This is a [Peripheral Access Crate](https://rust-embedded.github.io/book/start/registers.html)
for Qorvo PAC55XX microcontrollers.

This crate has been automatically generated from a custom SVD file
using [chiptool](https://github.com/embassy-rs/chiptool/).

## Supported chips

- **PAC55XX**: [Documentation](https://www.qorvo.com/products/power-solutions/intelligent-motor-controllers)

## Generation

Install [svdtools](https://github.com/rust-embedded/svdtools/) and
[form](https://github.com/djmcgill/form):

```sh
cargo install svdtools form
```

Make sure submodules are initialized:

```sh
git submodule update --init
```

Then, run:

```sh
make
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
