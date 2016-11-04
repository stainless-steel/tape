# Tape [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The library provides basic operations with tape archives (tar).

## [Documentation][documentation]

## Example

```rust
// tar -xf foo.tar -C bar
let (from, into) = ("foo.tar", "bar");
tape::open(from).unwrap().extract(into).unwrap();
```

## Acknowledgments

The library is based on [libtar][1] written by Mark D. Roth.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[1]: http://www.feep.net/libtar/

[documentation]: https://docs.rs/tape
[status-img]: https://travis-ci.org/stainless-steel/tape.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/tape
[version-img]: https://img.shields.io/crates/v/tape.svg
[version-url]: https://crates.io/crates/tape
