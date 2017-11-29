#[macro_use]
extern crate napi;
#[macro_use]
extern crate napi_derive;

use napi::{NapiArgs, NapiEnv, NapiResult, NapiUndefined};

#[derive(NapiArgs)]
struct InitializeArgs();

fn initialize(
    env: &NapiEnv,
    _args: InitializeArgs,
) -> NapiResult<NapiUndefined> {
    println!("Hello from the Rust land!");
    NapiUndefined::new(env)
}

napi_callback!(example_initialize, initialize);
