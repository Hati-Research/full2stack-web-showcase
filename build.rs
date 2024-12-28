use std::{error::Error, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    println!("rustc:rerun-if-changed=resources/tailwind.css");
    // TODO: In real program, change to a directory containing your html component definitions
    println!("rustc:rerun-if-changed=src/**/*.rs");
    let output = Command::new("npx")
        .arg("tailwindcss")
        .arg("-i")
        .arg("resources/tailwind.css")
        .arg("-o")
        .arg("static/stylesheet.css")
        .output()?;
    if !output.status.success() {
        panic!(
            "TailwindCSS compiler failed with exit code: {} and stderr: {}",
            output.status.code().unwrap_or(-1),
            String::from_utf8(output.stderr)?
        );
    }
    Ok(())
}
