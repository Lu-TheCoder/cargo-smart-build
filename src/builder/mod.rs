use crate::Commands;
use anyhow::{Context, Result};
use console::style;
use std::process::Command;

pub fn execute_command(
    command: Commands,
    package_name: &str,
    is_release: bool,
    features: &[String],
) -> Result<()> {
    let cargo_cmd = match command {
        Commands::Build => "build",
        Commands::Run => "run",
        Commands::Test => "test",
    };

    let mut cmd = Command::new("cargo");
    cmd.arg(cargo_cmd);
    cmd.arg("-p").arg(package_name);

    if is_release {
        cmd.arg("--release");
    }

    if !features.is_empty() {
        cmd.arg("--features");
        cmd.arg(features.join(","));
    }

    // Print command for transparency
    println!("\n{}:", style("Building").bold().green());
    println!("  - {}: {}", style("package").bold(), package_name);
    println!("  - {}: {}", style("profile").bold(), if is_release { "release" } else { "debug" });
    
    let feature_list = if features.is_empty() {
        "default".to_string()
    } else {
        features.join(", ")
    };
    println!("  - {}: {}", style("features").bold(), feature_list);

    let status = cmd
        .status()
        .with_context(|| format!("Failed to execute cargo {}", cargo_cmd))?;

    if !status.success() {
        anyhow::bail!("Command failed with exit code: {:?}", status.code());
    }

    Ok(())
}
