#[macro_use]
extern crate typed_key;

use typed_key::Key;

#[test]
fn key_is_copy() {
    const K: Key<Vec<String>> = typed_key!("foo");
    let x = K;
    let y = x;
    assert_eq!(x.name(), y.name());
}

#[test]
fn debug() {
    const FOO: Key<u32> = typed_key!("foo.bar");
    assert_eq!(format!("{:?}", FOO), r#"Key("foo.bar")"#);
}
