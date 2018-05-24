#![feature(const_fn)]

extern crate typed_key;

use typed_key::Key;

#[test]
fn key_is_copy() {
    const K: Key<u32> = Key::new("foo.bar");
    let x = K;
    let y = x;
    assert_eq!(x.name(), y.name());
}
