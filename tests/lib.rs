extern crate tar;

#[test]
fn extract() {
    use std::io::TempDir;
    use std::io::fs::PathExtensions;

    let foo = Path::new("tests").join_many(["fixtures", "foo.tar"]);
    assert!(foo.exists());

    let dir = TempDir::new("tar").unwrap();

    let tar = tar::open(&foo).unwrap();
    assert!(tar.extract(dir.path()).is_ok());

    let bar = dir.path().join("bar.txt");
    assert!(bar.exists());
}
