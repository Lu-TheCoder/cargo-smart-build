use anyhow::Result;

mod builder;
mod cargo_project;
mod config;
mod error;
mod features;
mod package_selector;
mod ui;

fn main() -> Result<()> {
    //~ Load last config
    let last_config = config::load_config();

    let metadata = cargo_project::load_metadata()?;
    let default_package = last_config.as_ref().map(|config| config.package.clone());
    let package = package_selector::select_package(&metadata, default_package)?;

    let features = features::extract_features(package);

    //~ Use previous build choices as defaults if available
    let default_release = last_config.as_ref().map(|config| config.release);
    let default_features = last_config
        .as_ref()
        .map(|c| c.features.as_slice())
        .unwrap_or(&[]);

    let is_release = ui::select_build_mode(default_release)?;
    let selected_features = ui::select_features(&features, default_features)?;

    builder::run_build(&package.name, is_release, &selected_features)?;

    //~ Save current selection
    let current_config = config::BuildConfig {
    	package: package.name.to_string(),
     	release: is_release,
      	features: selected_features,
    };
    config::save_config(&current_config);

    Ok(())
}
