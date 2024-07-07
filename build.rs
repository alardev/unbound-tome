use std::process::Command;

fn main() {
    Command::new("bunx")
        .arg("tailwindcss")
        .arg("-i")
        .arg("./styles/input.css")
        .arg("-o")
        .arg("./assets/tailwind.css")
        .arg("--minify")
        .output()
        .expect("failed to generate styles!");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=NONE");
}