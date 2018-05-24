# typed_key

[![Build Status](https://travis-ci.org/matklad/typed_key.svg?branch=master)](https://travis-ci.org/matklad/typed_key)
[![Crates.io](https://img.shields.io/crates/v/typed_key.svg)](https://crates.io/crates/typed_key)
[![API reference](https://docs.rs/typed_key/badge.svg)](https://docs.rs/typed_key/)


Strongly-typed string keys for configuration.

A frequent task is to extract a typed value from an untyped `Map<String, Object>`.
Typically, this is done via string keys: `let port: u32 = map.get("port")?.parse()?`.
A slightly more type-safe approach is to associate certain types with corresponding
string constants:

```rust
#[macro_use]
extern crate typed_key;
use typed_key::Key;

// `PORT` is basically `"port"` string with associated `u32` type.
const PORT: Key<u32> = typed_key!("port");
```

This crate provides basic building block for such strongly-typed strings.
See [example] for a complete example of reading configuration, and the [blog post]
for a more long winded explanation of the pattern.

[example]: https://github.com/matklad/typed_key/blob/master/examples/serde.rs
[blog post]: https://matklad.github.io/2018/05/24/typed-key-pattern.html

Using string keys is totally fine for small isolated cases, but if this pattern
is pervasive, `typed_key` can provide the following benefits:

  * Documentation: all possible keys are declared in one place, specify
    their type explicitly, and can have documentation comments.
  * Type safety: because each key carries its type, it's impossible to read
    the value of wrong type. Unlike string keys, typed keys never need a turbofish
    operator.
  * Typo safety: you can't misspell a type key on the call site.
  * Navigation: with typed_keys, you can use "go to definition", "find usages", and
    refactor without fear.
