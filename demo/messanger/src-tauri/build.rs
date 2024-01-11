use std::process::Command;

fn main() {
    Command::new("typeshare")
        .arg("./")
        .arg("--lang=typescript")
        .arg("--output-file=my_typescript_definitions.ts").status().unwrap();
    tauri_build::build()
}
