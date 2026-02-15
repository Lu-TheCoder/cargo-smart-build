use crate::Commands;
use anyhow::{Context, Result};
use console::style;
use std::process::Command;

pub fn execute_command(
    command: Commands,
    package_name: &str,
    is_release: bool,
    target: Option<&str>,
    features: &[String],
) -> Result<()> {
    let cargo_cmd = match command {
        Commands::Build => "build",
        Commands::Run => "run",
        Commands::Test => "test",
    };

    let mut program = "cargo";

    if let Some(_t) = target {
        //~ If target is specified, we should use cross
        program = "cross";

        //~ Check if available
        if Command::new("cross").arg("--version").output().is_err() {
            println!("{}", style("Cross is not installed.").yellow());
            let install = dialoguer::Confirm::new()
                .with_prompt("Do you want to install 'cross' via cargo?")
                .default(true)
                .interact()?;

            if install {
                let status = Command::new("cargo")
                    .args(["install", "cross"])
                    .status()
                    .context("Failed to install cross")?;

                if !status.success() {
                    anyhow::bail!("Failed to install cross. Aborting.");
                }
            } else {
                anyhow::bail!("Cross is required for cross-compilation. Aborting.");
            }
        }

        //~ Check for docker
        if Command::new("docker").arg("info").output().is_err() {
             anyhow::bail!("Docker is not running or not installed. Cross-compilation requires Docker.");
        }
    }

    let mut cmd = Command::new(program);
    cmd.arg(cargo_cmd);
    cmd.arg("-p").arg(package_name);

    if is_release {
        cmd.arg("--release");
    }

    if let Some(t) = target {
        cmd.arg("--target").arg(t);
    }

    if !features.is_empty() {
        cmd.arg("--features");
        cmd.arg(features.join(","));
    }

    // Print command for transparency
    println!("\n{}:", style("Building").bold().green());
    println!("  - {}: {}", style("package").bold(), package_name);
    println!("  - {}: {}", style("profile").bold(), if is_release { "release" } else { "debug" });
    println!("  - {}: {}", style("target").bold(), target.unwrap_or("native"));
    println!("  - {}: {}", style("runner").bold(), program); 
    
    let feature_list = if features.is_empty() {
        "default".to_string()
    } else {
        features.join(", ")
    };
    println!("  - {}: {}", style("features").bold(), feature_list);

    if matches!(command, Commands::Run) && target.is_some() {
        println!("{}", style("\nWarning: You are attempting to run a cross-compiled binary. This relies on 'cross' handling the emulation.").yellow());
    }

    let status = cmd
        .status()
        .with_context(|| format!("Failed to execute {} {}", program, cargo_cmd))?;

    if !status.success() {
        anyhow::bail!("Command failed with exit code: {:?}", status.code());
    }

    Ok(())
}
