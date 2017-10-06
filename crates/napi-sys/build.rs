extern crate bindgen;

#[cfg(unix)]
extern crate which;

use std::env;
use std::path::PathBuf;

#[cfg(unix)]
use which::which;

#[cfg(windows)]
use std::process::Command;

#[cfg(unix)]
fn find_node_api_header() -> Result<String, &'static str> {
    Ok(
        which("node")?
            .as_path()
            .parent()
            .ok_or("Could not go up from Node.js executable location")?
            .parent()
            .ok_or("Could not go up from Node.js bin directory")?
            .join("include")
            .join("node")
            .join("node_api.h")
            .to_str()
            .ok_or("Could not convert path to string")?
            .to_owned(),
    )
}

#[cfg(windows)]
fn find_node_api_header() -> Result<String, &'static str> {
    Command::new("cmd")
        .args(&["/C", "node-gyp", "install"])
        .output()
        .or(Err("Could not run node-gyp install"))?;

    let node_version_output = Command::new("cmd")
        .args(&["/C", "node", "--version"])
        .output()
        .or(Err("Could not run node --version"))?
        .stdout;

    let node_version = String::from_utf8(node_version_output).or(Err(
        "Could not parse node --version output",
    ))?;

    let home_dir = env::var("USERPROFILE").or(
        Err("Could not find user directory"),
    )?;

    let mut path = PathBuf::from(home_dir);
    path.push(".node_gyp");
    path.push(node_version.trim());
    path.push("include");
    path.push("node");
    path.push("node_api.h");

    Ok(
        path.to_str()
            .ok_or("Could not convert path to string")?
            .to_owned(),
    )
}

fn main() {
    let header_path = find_node_api_header().unwrap();

    let bindings = bindgen::Builder::default()
        .header(header_path)
        .hide_type("max_align_t")
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
