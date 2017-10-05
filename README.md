# XDR serialization and deserialization for Serde

This crate implements serialization and deserialization of the [External Data
Representation Standard (XDR)](https://tools.ietf.org/html/rfc1014) for the
[Serde](https://serde.rs/) serialization and deserialization framework.

## Usage

To use the official version publish in [crates.io](https://crates.io/), add the
following to your `Cargo.toml` file:

    [dependencies]
    serde-xdr = "0.1"

To serialize and deserialize data, you can use the provided helper functions:

 - `xdr_serde::from_reader(&mut reader) -> Result<T>`
 - `xdr_serde::to_bytes(&object_to_serialize) -> Result<Vec<u8>>`
 - `xdr_serde::to_writer(&mut writer, &object_to_serialize) -> Result<()>`

## Status

This crate should not be considered stable before more thorough real-world tests
have been made. If you find any bugs or inconsistencies, please report them as
GitHub issues.

One thing that is currently lacking tests is serialization and deserialization
failure conditions.

Documentation also could be improved.
