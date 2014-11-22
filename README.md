# Tape [![Build Status][travis-svg]][travis-url]

The library provides basic operations with tape archives (tar).

## [Documentation][docs]

## Example

In `Cargo.toml`:

```toml
[dependencies]
tape = "0.0.1"
```

In `main.rs`:

```rust
extern crate tape;

fn main() {
    // tar -xf foo.tar -C bar
    let archive = tape::open(&Path::new("foo.tar")).unwrap();
    archive.extract(&Path::new("bar"));
}
```

## Acknowledgments

The library is based on [libtar][1] written by Mark D. Roth.

## Contributing

1. Fork the project.
2. Implement your idea.
3. Create a pull request.

[1]: http://www.feep.net/libtar/

[travis-svg]: https://travis-ci.org/stainless-steel/tape.svg?branch=master
[travis-url]: https://travis-ci.org/stainless-steel/tape
[docs]: https://stainless-steel.github.io/tape
