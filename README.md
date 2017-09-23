# XDR serialization and deserialization for Serde

This crate implements serialization and deserialization of the [External Data
Representation Standard (XDR)](https://tools.ietf.org/html/rfc1014) for the
[Serde](https://serde.rs/) serialization and deserialization framework.

## Usage

Before an official first version is published to
[crates.io](https://crates.io/), a snapshot version should be used from this
repository. You can add the following to you `Cargo.toml` file:

    [dependencies]
    serde-xdr = { git = "https://github.com/jvff/serde-xdr.git" }

To serialize and deserialize data, you can use the provided helper functions:

 - `xdr_serde::from_reader<'de, R: Read, T: Deserialize<'de>>(reader: &'de mut R) -> Result<T>`
 - `xdr_serde::to_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>>`
 - `xdr_serde::to_writer<W: Write, T: Serialize>(writer: &mut W, value: &T) -> Result<()>`

## Status

This crate should not be considered stable before more thorough real-world tests
have been made. If you find any bugs or inconsistencies, please report them as
GitHub issues.

One thing that is currently lacking tests is serialization and deserialization
failure conditions.
