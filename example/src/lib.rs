#![feature(plugin)]
#![plugin(napi_codegen)]

extern crate napi;

use napi::sys::{napi_callback_info, napi_env, napi_value, napi_get_undefined};

use std::mem;

#[napi_callback("initialize")]
fn initialize(
    env: napi_env,
    _info: napi_callback_info,
) -> napi_value {
    println!("Hello from the Rust land!");

    unsafe {
        let mut result: napi_value = mem::uninitialized();
        napi_get_undefined(env, &mut result);

        result
    }
}
