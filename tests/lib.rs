#![feature(fs, path)]

extern crate tape;
extern crate temporary;

use std::path::PathBuf;
use temporary::Directory;

#[test]
fn extract() {
    use std::fs::PathExt;

    let mut foo = PathBuf::new("tests");
    foo.push("fixtures");
    foo.push("foo.tar");

    let dir = Directory::new("tape").unwrap();

    let archive = tape::open(&foo).unwrap();
    assert!(archive.extract(dir.path()).is_ok());

    let bar = dir.path().join("bar.txt");
    assert!(bar.exists());
}
