extern crate tar;

#[test]
fn extract_all() {
    use std::io::TempDir;
    use std::io::fs::PathExtensions;

    let foo = Path::new("tests").join_many(["fixtures", "foo.tar"]);
    assert!(foo.exists());

    let dir = TempDir::new("tar").unwrap();

    let tar = tar::Archive::open(&foo).unwrap();
    tar.extract_all(dir.path()).unwrap();

    let bar = dir.path().join("bar.txt");
    assert!(bar.exists());
}
