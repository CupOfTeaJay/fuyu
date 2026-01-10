/*
    Fuyu
    Copyright (c) 2026-2026 whoami™ LLC

    TODO: document.
*/

use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// TODO: Document.
fn compile(mods: &Path) {
    let output = Command::new("cargo")
        .args([
            "build",
            "--manifest-path",
            mods.join("Cargo.toml").to_str().unwrap(),
        ])
        .output()
        .unwrap();
    // println!("status: {}", output.status);
    let _ = io::stdout().write_all(&output.stdout);
    let _ = io::stderr().write_all(&output.stderr);
}

fn main() {
    compile(Path::new("mods"))
}
