# XDR serialization and deserialization for Serde

This crate implements serialization and deserialization of the [External Data
Representation Standard (XDR)][1] for the [Serde][2] serialization and
deserialization framework.

## Usage

To use the official version published on [crates.io][3], add the following to
your `Cargo.toml` file:

    [dependencies]
    serde-xdr = "0.4"

To serialize and deserialize data, you can use the provided helper functions:

 - `xdr_serde::from_bytes(&mut bytes) -> Result<T>`
 - `xdr_serde::from_reader(&mut reader) -> Result<T>`
 - `xdr_serde::to_bytes(&object_to_serialize) -> Result<Vec<u8>>`
 - `xdr_serde::to_writer(&mut writer, &object_to_serialize) -> Result<()>`

A more complete example is available in the [documentation][4].

[1]: https://tools.ietf.org/html/rfc1014
[2]: https://serde.rs/
[3]: https://crates.io/crates/serde-xdr
[4]: https://docs.rs/serde-xdr/0.4.0/serde_xdr/

## Status

This crate should not be considered stable before more thorough real-world tests
have been made. If you find any bugs or inconsistencies, please report them as
GitHub issues.

One thing that is currently lacking tests is serialization and deserialization
failure conditions.

Documentation also could be improved.
