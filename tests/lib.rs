#![feature(fs, path)]

extern crate tape;
extern crate temporary;

use std::path::PathBuf;
use temporary::Directory;

#[test]
fn extract() {
    use std::fs::PathExt;

    let foo = PathBuf::new("tests").join("fixtures").join("foo.tar");
    let directory = Directory::new("tape").unwrap();

    let archive = tape::open(&foo).unwrap();
    assert!(archive.extract(directory.path()).is_ok());

    let bar = directory.path().join("bar.txt");
    assert!(bar.exists());
}
