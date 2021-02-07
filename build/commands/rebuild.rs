use std::io::Error;
use std::process::Command;
use std::env;

pub fn rebuild() -> Result<(), Error> {
    println!();

    let path = env::current_dir().unwrap();

    let child = Command::new("cargo")
        .current_dir(path)
        .arg("build")
        .arg("--release")
        .env("CARGO_TARGET_DIR", ".melonbuild")
        .spawn()
        .expect("Build failed to start.");

    child.wait_with_output()?;

    println!();
    
    Ok(())
}