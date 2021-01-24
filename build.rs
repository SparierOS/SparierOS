// SPDX-License-Identifier: MIT

use std::env;

fn main() {
    // The path to the Linker Script is set by the Makefile
    let linker_script = env::var("LINKER_SCRIPT").unwrap();

    println!("cargo:rerun-if-changed={}", linker_script);
}
