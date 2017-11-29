# Node.js N-API for Rust! [work in progress]

[![Build Status][travis-badge]][travis-url]

High-level N-API bindings for Node.js addons written in Rust.

This project is covered by a [Code of Conduct](CODE_OF_CONDUCT.md).

## Crates

* `napi-sys`: low-level bindings to N-API generated from
  [`node_api.h`](https://github.com/nodejs/node/blob/master/src/node_api.h)
  using [`bindgen`](https://github.com/rust-lang-nursery/rust-bindgen).
* `napi`: high-level and rusty wrappers around `napi-sys`.
* `napi-derive`: contains a procedural macro that allows to construct typesafe
   structures that represent function parameters from JavaScript function call
   arguments and automatically validate them.

[travis-badge]: https://travis-ci.org/aqrln/napi-rs.svg?branch=master
[travis-url]: https://travis-ci.org/aqrln/napi-rs
