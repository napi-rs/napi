#[macro_use]
extern crate napi;
#[macro_use]
extern crate napi_derive;

use napi::{NapiArgs, NapiEnv, NapiNumber, NapiResult, NapiUndefined};

#[derive(NapiArgs)]
struct HelloArgs;

fn hello<'a>(env: &'a NapiEnv, _: &HelloArgs) -> NapiResult<NapiUndefined<'a>> {
    println!("Hello from the Rust land!");
    NapiUndefined::new(env)
}

#[derive(NapiArgs)]
struct AddArgs<'a> {
    first: NapiNumber<'a>,
    second: NapiNumber<'a>,
}

fn add<'a>(env: &'a NapiEnv, args: &AddArgs<'a>) -> NapiResult<NapiNumber<'a>> {
    let first = args.first.to_i32()?;
    let second = args.second.to_i32()?;
    NapiNumber::from_i32(env, first + second)
}

napi_callback!(example_hello, hello);
napi_callback!(example_add, add);
