use std::process::Command;

fn main() {
    Command::new("typeshare")
        .arg("./")
        .arg("--lang=typescript")
        .arg("--output-file=../src/rust_type_definitions.ts").status().unwrap();
    tauri_build::build()
}
