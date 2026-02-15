use anyhow::{Context, Result};
use dialoguer::{Select, Input};
use console::style;
use std::process::Command;

/// Returns a list of installed targets (e.g., "x86_64-unknown-linux-gnu")
pub fn get_installed_targets() -> Result<Vec<String>> {
    let output = Command::new("rustup")
        .arg("target")
        .arg("list")
        .arg("--installed")
        .output()
        .context("Failed to execute 'rustup target list --installed'")?;

    if !output.status.success() {
        // If rustup is not installed or fails, we might just return empty
        // but for now let's error out or return empty to fall back to native.
        return Ok(vec![]);
    }

    let stdout = String::from_utf8(output.stdout)?;
    let targets = stdout
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(targets)
}

/// Prompts the user to select a target.
/// Returns `None` for "Default (Native)", or `Some("target-triple")`.
pub fn select_target(default_target: Option<String>) -> Result<Option<String>> {
    let targets = get_installed_targets()?;
    
    // UI Options
    let mut options = vec![format!("{}", style("Default (Native)").bold())];
    
    // Add installed targets
    for target in &targets {
        if let Some(def) = &default_target {
            if def == target {
                 options.push(format!("{}", style(target).green().bold()));
                 continue;
            }
        }
        options.push(target.clone());
    }

    // Add option to install new target
    options.push(style("+ Add new target").dim().to_string());

    let default_index = if let Some(def) = &default_target {
        // +1 because "Default (Native)" is at index 0
        if let Some(pos) = targets.iter().position(|t| t == def) {
            pos + 1 
        } else {
            0
        }
    } else {
        0
    };

    let selection = Select::new()
        .with_prompt("Select target")
        .items(&options)
        .default(default_index)
        .interact()?;

    if selection == 0 {
        return Ok(None); // Native
    }

    if selection == options.len() - 1 {
        // "+ Add new target" selected
        let new_target: String = Input::new()
            .with_prompt("Enter target triple (e.g., wasm32-unknown-unknown)")
            .interact_text()?;

        // Attempt to install
        println!("Installing target: {}", new_target);
        let status = Command::new("rustup")
            .arg("target")
            .arg("add")
            .arg(&new_target)
            .status()
            .context("Failed to run rustup target add")?;

        if !status.success() {
            println!("{}", style("Failed to install target.").red());
            return Ok(None); // Fallback to native on failure
        }
        
        return Ok(Some(new_target));
    }

    // Map selection index back to target string
    // Index 0 is Native.
    // Index 1..=targets.len() correspond to targets[0..]
    let target_str = targets[selection - 1].clone();
    Ok(Some(target_str))
}
