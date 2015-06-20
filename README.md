# Tape [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The library provides basic operations with tape archives (tar).

## [Documentation][docs]

## Example

```rust
// tar -xf foo.tar -C bar
let (from, into) = ("foo.tar", "bar");
tape::open(from).unwrap().extract(into).unwrap();
```

## Acknowledgments

The library is based on [libtar][1] written by Mark D. Roth.

## Contributing

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[1]: http://www.feep.net/libtar/

[version-img]: https://img.shields.io/crates/v/tape.svg
[version-url]: https://crates.io/crates/tape
[status-img]: https://travis-ci.org/stainless-steel/tape.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/tape
[docs]: https://stainless-steel.github.io/tape
