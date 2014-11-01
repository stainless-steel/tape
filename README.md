# Tar [![Build Status][travis-svg]][travis-url]

The library provides an interface to [libtar][1].

## Usage

In `Cargo.toml`:

```toml
[dependencies.tar]
git = "https://github.com/IvanUkhov/rs-tar"
```

In `main.rs`:

```rust
extern crate tar;

fn main() {
    let tar = tar::Archive::open(&Path::new("foo.tar")).unwrap();
    tar.extract_all(&Path::new("bar"));
}
```

## Contributing

1. Fork the project.
2. Implement your feature.
3. Create a pull request.

[1]: http://www.feep.net/libtar/

[travis-svg]: https://travis-ci.org/IvanUkhov/rs-tar.svg?branch=master
[travis-url]: https://travis-ci.org/IvanUkhov/rs-tar
