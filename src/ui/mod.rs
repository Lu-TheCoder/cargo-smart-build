use anyhow::Result;
use dialoguer::{MultiSelect, Select};
use console::style;

pub fn select_build_mode(default_release: Option<bool>) -> Result<bool> {
    let modes = vec!["Debug", "Release"];
    let display_modes: Vec<String> = modes
        .iter()
        .map(|&mode| {
            let is_default = if let Some(release) = default_release {
                (mode == "Release") == release
            } else {
                mode == "Debug" // Default is Debug if no config (index 0)
            };

            if is_default {
                format!("{}", style(mode).green())
            } else {
                mode.to_string()
            }
        })
        .collect();

    let default_index = if let Some(release) = default_release {
    	if release { 1 } else { 0 }
    } else {
    	0
    };

    let selection = Select::new()
        .with_prompt("Select build mode")
        .items(&display_modes)
        .default(default_index)
        .interact()?;

    Ok(selection == 1)
}

pub fn select_features(features: &[String], default_features: &[String]) -> Result<Vec<String>> {
    if features.is_empty() {
        return Ok(vec![]);
    }

    //~ Compute booleans of preselected features
    let defaults: Vec<bool> = features
        .iter()
        .map(|feature| default_features.contains(feature))
        .collect();

    //~ Create display names with highlighting for defaults
    let display_items: Vec<String> = features
        .iter()
        .map(|feature| {
            if default_features.contains(feature) {
                format!("{}", style(feature).green())
            } else {
                feature.clone()
            }
        })
        .collect();

    let selections = MultiSelect::new()
        .with_prompt("Select features to enable")
        .items(&display_items)
        .defaults(&defaults)
        .interact()?;

    Ok(selections
        .into_iter()
        .map(|i| features[i].clone())
        .collect())
}
