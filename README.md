[github-ci-linux]: https://github.com/FedericoBruzzone/ende/actions/workflows/build-linux.yml
[github-ci-linux-shield]: https://github.com/FedericoBruzzone/ende/actions/workflows/build-linux.yml/badge.svg
[github-ci-windows]: https://github.com/FedericoBruzzone/ende/actions/workflows/build-windows.yml
[github-ci-windows-shield]: https://github.com/FedericoBruzzone/ende/actions/workflows/build-windows.yml/badge.svg
[github-ci-macos]: https://github.com/FedericoBruzzone/ende/actions/workflows/build-macos.yml
[github-ci-macos-shield]: https://github.com/FedericoBruzzone/ende/actions/workflows/build-macos.yml/badge.svg
[crates-io]: https://crates.io/crates/ende
[crates-io-shield]: https://img.shields.io/crates/v/ende
[github-license]: https://github.com/FedericoBruzzone/ende/blob/main/LICENSE
[github-license-shield]: https://img.shields.io/github/license/FedericoBruzzone/ende
[total-lines]: https://github.com/FedericoBruzzone/ende
[total-lines-shield]: https://tokei.rs/b1/github/FedericoBruzzone/ende?type=Rust,Python
[creates-io-downloads]: https://crates.io/crates/ende
[creates-io-downloads-shield]: https://img.shields.io/crates/d/ende.svg
[dependents]: https://crates.io/crates/ende/reverse_dependencies
[dependents-shield]: https://img.shields.io/librariesio/dependents/cargo/ende
[documentation]: https://docs.rs/ende
[documentation-shield]: https://docs.rs/ende/badge.svg

[![GitHub CI Linux][github-ci-linux-shield]][github-ci-linux]
[![GitHub CI Windows][github-ci-windows-shield]][github-ci-windows]
[![GitHub CI macOS][github-ci-macos-shield]][github-ci-macos]
[![Crates.io][crates-io-shield]][crates-io]
[![GitHub License][github-license-shield]][github-license]
[![Crates.io Downloads][creates-io-downloads-shield]][creates-io-downloads]
[![][total-lines-shield]][total-lines]
[![dependents][dependents-shield]][dependents]
[![Documentation][documentation-shield]][documentation]

# Encdec

A library for encoding/decoding unicode/utf-8/utf-16(ucs-2) code points.

## Examples

**Encoding/decoding unicode/UTF-8 code points**

```rust
let v: Vec<u32> = vec![0x10348 /*...*/]; // Array of unicode code points
let utf8_vec: Vec<u8> = utf8::encode_in_utf8(&v);
utf8::print_utf8_b(&utf8_vec);
let unicode_vec: Vec<u32> = utf8::decode_from_utf8(&utf8_vec);
unicode::print_unicode_b(&v);
```

```shell
--------------- UTF-8 of "𐍈" ---------------
Hex: [f0, 90, 8d, 88]
Bin: ["11110000", "10010000", "10001101", "10001000"]
Dec: [240, 144, 141, 136]
--------------------------------------------

--------------- UNICODE of "𐍈" ---------------
Hex: [10348]
Bin: ["10000001101001000"]
Dec: [66376]
----------------------------------------------
```

**Encoding/decoding unicode/UTF-16 code points**

```rust
let v: Vec<u32> = vec![0x10001 /*...*/]; // Array of unicode code points
let utf16_vec: Vec<u16> = utf16::encode_in_utf16(&v);
utf16::print_utf16_b(&utf16_vec);
let unicode_vec: Vec<u32> = utf16::decode_from_utf16(&utf16_vec);
unicode::print_unicode_b(&v);
```

```shell
--------------- UTF-16 of "𐀁" ---------------
Hex: [d800, dc01]
Bin: ["1101100000000000", "1101110000000001"]
Dec: [55296, 56321]
---------------------------------------------

--------------- UNICODE of "𐀁" ---------------
Hex: [10001]
Bin: ["10000000000000001"]
Dec: [65537]
----------------------------------------------
```

## Building

You can use `just`, `make` or `cargo`,  as build tools.
If you want to use `cargo`, please make sure to read the `Justfile` or the `Makefile` to understand the flags used for each command.
Here are the available commands:

```text
just COMMAND
make COMMAND

COMMAND:
  all    # fmt, clippy, test, build
  build  # Build the project
  run    # Run the project
  fmt    # Format the code
  clippy # Run clippy
  test   # Run the tests
  clean  # Clean the project
  help   # Print the help message
```

## Contributing

Contributions to this project are welcome! If you have any suggestions, improvements, or bug fixes, feel free to submit a pull request.

## License

This repository is licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0). Please review the license file provided in the repository for more information regarding the terms and conditions of the license.

## Contact

- Email:
  - [federico.bruzzone.i@gmail.com]
  - [federico.bruzzone@studenti.unimi.it]
- GitHub:
  - [FedericoBruzzone](https://github.com/FedericoBruzzone)

