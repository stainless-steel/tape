# Tar [![Build Status][travis-svg]][travis-url]

The library provides basic operations with tar archives.

## [Documentation][docs]

## Example

In `Cargo.toml`:

```toml
[dependencies.tar]
git = "https://github.com/stainless-steel/tar"
```

In `main.rs`:

```rust
extern crate tar;

fn main() {
    // tar -xf foo.tar -C bar
    let tar = tar::open(&Path::new("foo.tar")).unwrap();
    tar.extract(&Path::new("bar"));
}
```

## Acknowledgments

The library is based on [libtar][1] written by Mark D. Roth.

## Contributing

1. Fork the project.
2. Implement your idea.
3. Create a pull request.

[1]: http://www.feep.net/libtar/

[travis-svg]: https://travis-ci.org/stainless-steel/tar.svg?branch=master
[travis-url]: https://travis-ci.org/stainless-steel/tar
[docs]: https://stainless-steel.github.io/tar
