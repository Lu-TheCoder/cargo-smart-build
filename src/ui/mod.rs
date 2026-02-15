use anyhow::Result;
use dialoguer::{MultiSelect, Select};

pub fn select_build_mode(default_release: Option<bool>) -> Result<bool> {
    let modes = vec!["Debug", "Release"];
    let default_index = if let Some(release) = default_release {
    	if release { 1 } else { 0 }
    } else {
    	0
    };

    let selection = Select::new()
        .with_prompt("Select build mode")
        .items(&modes)
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

    let selections = MultiSelect::new()
        .with_prompt("Select features to enable")
        .items(features)
        .defaults(&defaults)
        .interact()?;

    Ok(selections
        .into_iter()
        .map(|i| features[i].clone())
        .collect())
}
