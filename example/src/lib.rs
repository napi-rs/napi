#![feature(plugin)]
#![plugin(napi_codegen)]

extern crate napi;

use napi::{NapiEnv, NapiResult, NapiUndefined};

#[napi_callback("initialize")]
fn initialize(env: &NapiEnv) -> NapiResult<NapiUndefined> {
    println!("Hello from the Rust land!");
    NapiUndefined::new(env)
}
