#![allow(unstable)]

extern crate tape;

#[test]
fn extract() {
    use std::io::TempDir;
    use std::io::fs::PathExtensions;

    let foo = Path::new("tests").join_many(&["fixtures", "foo.tar"]);
    assert!(foo.exists());

    let dir = TempDir::new("tape").unwrap();

    let archive = tape::open(&foo).unwrap();
    assert!(archive.extract(dir.path()).is_ok());

    let bar = dir.path().join("bar.txt");
    assert!(bar.exists());
}
