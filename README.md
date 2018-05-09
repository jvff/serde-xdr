# XDR serialization and deserialization for Serde

This crate implements serialization and deserialization of the [External Data
Representation Standard (XDR)][1] for the [Serde][2] serialization and
deserialization framework.

## Usage

To use the official version published on [crates.io][3], add the following to
your `Cargo.toml` file:

    [dependencies]
    serde-xdr = "0.5"

To serialize and deserialize data, you can use the provided helper functions:

 - `xdr_serde::from_bytes(&mut bytes) -> Result<T>`
 - `xdr_serde::from_reader(&mut reader) -> Result<T>`
 - `xdr_serde::to_bytes(&object_to_serialize) -> Result<Vec<u8>>`
 - `xdr_serde::to_writer(&mut writer, &object_to_serialize) -> Result<()>`

A more complete example is available in the [documentation][4].

[1]: https://tools.ietf.org/html/rfc1014
[2]: https://serde.rs/
[3]: https://crates.io/crates/serde-xdr
[4]: https://docs.rs/serde-xdr/0.5.0/serde_xdr/

## Data Type Map

The table below describes the mapping between Rust types and XDR.

| Rust type | XDR Reference | XDR Details |
|-----------|---------------|-------------|
| [`bool`][rust-bool] | [boolean][xdr-bool] | a 32-bit MSB integer that's zero or one |
| [`i8`][rust-i8] | [integer][xdr-integer] | a 32-bit MSB integer |
| [`i16`][rust-i16] | [integer][xdr-integer] | a 32-bit MSB integer |
| [`i32`][rust-i32] | [integer][xdr-integer] | a 32-bit MSB integer |
| [`i64`][rust-i64] | [hyper integer][xdr-hyper] | a 64-bit MSB integer |
| [`u8`][rust-u8] | [unsigned integer][xdr-unsigned] | an unsigned 32-bit MSB integer |
| [`u16`][rust-u16] | [unsigned integer][xdr-unsigned] | an unsigned 32-bit MSB integer |
| [`u32`][rust-u32] | [unsigned integer][xdr-unsigned] | an unsigned 32-bit MSB integer |
| [`u64`][rust-u64] | [unsigned hyper integer][xdr-hyper] | an unsigned 64-bit MSB integer |
| [`f32`][rust-f32] | [floating-point][xdr-float] | a 32-bit MSB floating-point number |
| [`f64`][rust-f64] | [double-precision floating-point][xdr-double] | a 64-bit MSB floating-point number |
| [`char`][rust-char] | [unsigned integer][xdr-unsigned] | an unsigned 32-bit MSB integer |
| [`&str`][rust-str] [<sup>1</sup>][notes] | [string][xdr-string] | an unsigned 32-bit MSB integer representing the length, followed by one byte for each character of the string |
| [`String`][rust-string] [<sup>1</sup>][notes] | [string][xdr-string] | an unsigned 32-bit MSB integer representing the length, followed by one byte for each character of the string |
| [`Option<T>`][rust-option] | [optional-data][xdr-optional] | a 32-bit MSB integer that's zero or one, and if it's one it is followed by the serialization of `T` |
| [`()`][rust-unit] | [void][xdr-void] | no bytes are serialized |
| [`struct T;`][rust-unit-struct] | [void][xdr-void] | no bytes are serialized |
| [`struct T(A, B, ...)`][rust-tuple-struct] | [structure][xdr-structure] | each element in the tuple is serialized in sequence |
| [`struct T { _: A, _: B, ... }`][rust-struct] | [structure][xdr-structure] | each field is serialized in sequence in the order they were declared |
| [`(A, B, ...)`][rust-tuple] | [structure][xdr-structure] | each element in the tuple is serialized in sequence |
| [`enum`][rust-enum] [<sup>2</sup>][notes] | [discriminated union][xdr-union] | an unsigned 32-bit MSB integer representing the index of the variant (starting from zero), followed by the serialization of the variant |
| [`&[T]`][rust-slice] [<sup>3</sup>][notes] | [variable-length array][xdr-var-array] [<sup>4</sup>][notes] | an unsigned 32-bit MSB integer representing the length, followed by the serialization of each element |
| [`[T; N]`][rust-array] [<sup>3</sup>][notes] | [variable-length array][xdr-var-array] [<sup>5,6</sup>][notes] | an unsigned 32-bit MSB integer representing the length, followed by the serialization of each element |
| [`Vec<T>`][rust-vec] [<sup>3</sup>][notes] | [variable-length array][xdr-var-array] | an unsigned 32-bit MSB integer representing the length, followed by the serialization of each element |
| [`serde_bytes::Bytes`][serde_bytes-bytes] [<sup>3</sup>][notes] | [variable-length opaque data][xdr-var-opaque] | an unsigned 32-bit MSB integer representing the length, followed by the bytes with up to three bytes with zeros for padding |

### Notes

1. Must be valid ASCII and can't be longer than 2^32 - 1 characters.
2. Can't have more than 2^32 - 1 variants.
3. Can't have more than 2^32 - 1 elements.
4. Use [`serde_bytes`][serde_bytes] for a more efficient serialization.
5. Efficient mapping to [fixed-length array][xdr-fix-array] has not been implemented yet.
6. If efficient serialization of `[u8; N]` is desired, use [`serde_xdr::opaque_data::fixed_length`][fixed-length].

[fixed-length]: https://docs.rs/serde-xdr/*/serde_xdr/opaque_data/fixed_length/index.html
[notes]: #notes
[rust-bool]: https://doc.rust-lang.org/std/primitive.bool.html
[rust-i8]: https://doc.rust-lang.org/std/primitive.i8.html
[rust-i16]: https://doc.rust-lang.org/std/primitive.i16.html
[rust-i32]: https://doc.rust-lang.org/std/primitive.i32.html
[rust-i64]: https://doc.rust-lang.org/std/primitive.i64.html
[rust-u8]: https://doc.rust-lang.org/std/primitive.u8.html
[rust-u16]: https://doc.rust-lang.org/std/primitive.u16.html
[rust-u32]: https://doc.rust-lang.org/std/primitive.u32.html
[rust-u64]: https://doc.rust-lang.org/std/primitive.u64.html
[rust-f32]: https://doc.rust-lang.org/std/primitive.f32.html
[rust-f64]: https://doc.rust-lang.org/std/primitive.f64.html
[rust-char]: https://doc.rust-lang.org/std/primitive.char.html
[rust-str]: https://doc.rust-lang.org/std/primitive.str.html
[rust-unit]: https://doc.rust-lang.org/std/primitive.unit.html
[rust-slice]: https://doc.rust-lang.org/std/primitive.slice.html
[rust-array]: https://doc.rust-lang.org/std/primitive.array.html
[rust-tuple]: https://doc.rust-lang.org/nightly/std/primitive.tuple.html
[rust-string]: https://doc.rust-lang.org/std/string/struct.String.html
[rust-vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[rust-option]: https://doc.rust-lang.org/std/option/enum.Option.html
[rust-unit-struct]: https://doc.rust-lang.org/book/second-edition/ch05-01-defining-structs.html#unit-like-structs-without-any-fields
[rust-tuple-struct]: https://doc.rust-lang.org/book/second-edition/ch05-01-defining-structs.html#tuple-structs-without-named-fields-to-create-different-types
[rust-struct]: https://doc.rust-lang.org/book/second-edition/ch05-01-defining-structs.html
[rust-enum]: https://doc.rust-lang.org/book/second-edition/ch06-01-defining-an-enum.html
[serde_bytes]: https://docs.rs/serde_bytes/*/serde_bytes/
[serde_bytes-bytes]: https://docs.rs/serde_bytes/*/serde_bytes/struct.Bytes.html
[xdr-integer]: https://tools.ietf.org/html/rfc4506#section-4.1
[xdr-unsigned]: https://tools.ietf.org/html/rfc4506#section-4.2
[xdr-bool]: https://tools.ietf.org/html/rfc4506#section-4.4
[xdr-hyper]: https://tools.ietf.org/html/rfc4506#section-4.5
[xdr-float]: https://tools.ietf.org/html/rfc4506#section-4.6
[xdr-double]: https://tools.ietf.org/html/rfc4506#section-4.7
[xdr-var-opaque]: https://tools.ietf.org/html/rfc4506#section-4.10
[xdr-string]: https://tools.ietf.org/html/rfc4506#section-4.11
[xdr-fix-array]: https://tools.ietf.org/html/rfc4506#section-4.12
[xdr-var-array]: https://tools.ietf.org/html/rfc4506#section-4.13
[xdr-structure]: https://tools.ietf.org/html/rfc4506#section-4.14
[xdr-union]: https://tools.ietf.org/html/rfc4506#section-4.15
[xdr-void]: https://tools.ietf.org/html/rfc4506#section-4.16
[xdr-optional]: https://tools.ietf.org/html/rfc4506#section-4.19

## Status

This crate should not be considered stable before more thorough real-world tests
have been made. If you find any bugs or inconsistencies, please report them as
GitHub issues.

One thing that is currently lacking tests is serialization and deserialization
failure conditions.

Documentation also could be improved.
