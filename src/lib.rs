//! # typed_key
//!
//! A frequent task is to extract a typed value from an untyped `Map<String, Object>`.
//! Typically, this is done via string keys: `let port: u32 = map.get("port")?.parse()?`.
//! A slightly more type-safe approach is to associate certain types with corresponding
//! string constants:
//!
//! ```rust
//! #[macro_use]
//! extern crate typed_key;
//! use typed_key::Key;
//!
//! // `PORT` is basically `"port"` string with associated `u32` type.
//! const PORT: Key<u32> = typed_key!("port");
//!
//! # fn main() {}
//! ```
//!
//! This crate provides basic building block for such strongly-typed strings.
//! See [example] for a complete example of reading configuration, and the [blog post]
//! for a more long winded explanation of the pattern.
//!
//! [example]: https://github.com/matklad/typed_key/blob/master/examples/serde.rs
//! [blog post]: https://matklad.github.io/2018/05/24/typed_key_pattern.html
//!
//! Using string keys is totally fine for small isolated cases, but if this pattern
//! is pervasive, `typed_key` can provide the following benefits:
//!
//!   * Documentation: all possible keys are declared in one place, specify
//!     their type explicitly, and can have documentation comments.
//!   * Type safety: because each key carries its type, it's impossible to read
//!     the value of wrong type. Unlike string keys, typed keys never need a turbofish
//!     operator.
//!   * Typo safety: you can't misspell a type key on the call site.
//!   * Navigation: with typed_keys, you can use "go to definition", "find usages", and
//!     refactor without fear.

#![no_std]
#![cfg_attr(feature = "nightly", feature(const_fn))]

use core::{fmt, marker::PhantomData};

// neede for `typed_key!` macro to work for both std and no-std crates
#[doc(hidden)]
pub mod __reexports {
    pub use core::marker::PhantomData;
}

// A `Key<T>` is a string constant which additionally remembers type `T`.
pub struct Key<T> {
    // public for `typed_key!` macro
    #[doc(hidden)]
    pub __name: &'static str,

    // public for `typed_key!` macro
    #[doc(hidden)]
    pub __marker: PhantomData<T>,
}

impl<T> Copy for Key<T> {}

impl<T> Clone for Key<T> {
    fn clone(&self) -> Self {
        Key {
            __name: self.__name,
            __marker: PhantomData,
        }
    }
}

impl<T> fmt::Debug for Key<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Key({:?})", self.__name)
    }
}

impl<T> Key<T> {
    /// Construct new `Key` with the specified `name`.
    /// To use this function, enable the `nightly` feature of
    /// this crate and `#![feature(const_fn)]` to your crate.
    #[cfg(feature = "nightly")]
    pub const fn new(name: &'static str) -> Key<T> {
        Key {
            __name: name,
            __marker: PhantomData,
        }
    }

    /// String name of this `Key`.
    pub fn name(&self) -> &'static str {
        self.__name
    }
}

/// Constructs a new `Key` with a given name.
///
/// # Examples
///
/// ```rust
/// #[macro_use]
/// extern crate typed_key;
///
/// use std::net::IpAddr;
/// use typed_key::Key;
///
/// /// IP address of the server
/// const ADDR: Key<IpAddr> = typed_key!("addr");
///
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! typed_key {
    ($name:expr) => {{
        $crate::Key {
            __name: $name,
            __marker: $crate::__reexports::PhantomData,
        }
    }};
}
