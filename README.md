# Node.js N-API for Rust! [work in progress]

[![Build Status][travis-badge]][travis-url]

## Crates

* `napi-sys`: low-level bindings to N-API generated from
  [`node_api.h`](https://github.com/nodejs/node/blob/master/src/node_api.h)
  using [`bindgen`](https://github.com/rust-lang-nursery/rust-bindgen).
* `napi`: high-level and rusty wrappers around `napi-sys`.
* `napi-codegen`: a compiler plugin designed for you to be able to write a
  regular Rust function that takes JavaScript values as arguments and returns a
  `NapiResult`, put an annotation, and be good to go.  It's not clear at this
  point whether we will continue taking this route, though.  On the second
  thought, pretty function signatures don't seem worth diving into the hassle
  of maintaining compatibility with the latest nightly compiler and requiring
  using it.  Good old macros and just a little bit of boilerplate code will
  solve the problem as well.

[travis-badge]: https://travis-ci.org/aqrln/napi-rs.svg?branch=master
[travis-url]: https://travis-ci.org/aqrln/napi-rs
