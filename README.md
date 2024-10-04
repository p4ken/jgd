# jgd

[![crates.io](https://img.shields.io/crates/v/jgd.svg)](https://crates.io/crates/jgd)

Transform geodetic datums used in Japan.

## Getting started

```sh
cargo add jgd
```

## Examples

```rs
use jgd::{LatLon, Tokyo};

let LatLon(lat, lon) = Tokyo::new(LatLon(35.0, 135.0))?
    .to_jgd2000()
    .to_jgd2011()
    .degrees();
```

## [API documentation](https://docs.rs/jgd/)

## [MIT license](LICENSE.md)
