liberasurecode
==============

[![Crates.io: liberasurecode](https://img.shields.io/crates/v/liberasurecode.svg)](https://crates.io/crates/liberasurecode)
[![Documentation](https://docs.rs/liberasurecode/badge.svg)](https://docs.rs/liberasurecode)
[![Build Status](https://travis-ci.org/frugalos/liberasurecode.svg?branch=master)](https://travis-ci.org/frugalos/liberasurecode)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Rust wrapper for [openstack/liberasurecode].

[Documentation](https://docs.rs/liberasurecode)

[openstack/liberasurecode]: https://github.com/openstack/liberasurecode


Prerequisites to Build
----------------------

This crate requires the following packages for building [openstack/liberasurecode] in the build script:
- C compiler (e.g., `gcc`)
- `git`
- `make`
- `automake`
- `autoconf`
- `libtool`

For example, on Ubuntu, you can install those by executing the following command:
```console
$ sudo apt install gcc git make automake autoconf libtool
```


Examples
--------

Basic usage:
```rust
use liberasurecode::{ErasureCoder, Error};

let mut coder = ErasureCoder::new(4, 2)?;
let input = vec![0, 1, 2, 3];

// Encodes `input` to data and parity fragments
let fragments = coder.encode(&input)?;

// Decodes the original data from the fragments (or a part of those)
assert_eq!(Ok(&input), coder.decode(&fragments[0..]).as_ref());
assert_eq!(Ok(&input), coder.decode(&fragments[1..]).as_ref());
assert_eq!(Ok(&input), coder.decode(&fragments[2..]).as_ref());
assert_eq!(Err(Error::InsufficientFragments), coder.decode(&fragments[3..]));
```
