extern crate tape;
extern crate temporary;

use std::fs;
use std::path::PathBuf;

use temporary::Directory;

#[test]
fn extract() {
    let foo = PathBuf::from("tests").join("fixtures").join("foo.tar");
    let directory = Directory::new("tape").unwrap();

    let archive = tape::open(foo).unwrap();
    assert!(archive.extract(directory.path()).is_ok());

    let bar = directory.path().join("bar.txt");
    assert!(fs::metadata(bar).is_ok());
}
