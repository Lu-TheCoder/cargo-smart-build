use anyhow::{Context, Result};
use std::process::Command;

pub fn run_build(package_name: &str, is_release: bool, features: &[String]) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build");

    cmd.arg("-p");
    cmd.arg(package_name);

    if is_release {
        cmd.arg("--release");
    }

    if !features.is_empty() {
        cmd.arg("--features");
        cmd.arg(features.join(" "));
    }

    println!("\nRunning: {:?}", cmd);

    let status = cmd.status().context("Failed to execute cargo build")?;

    if !status.success() {
        println!("Build failed.");
    }

    Ok(())
}
