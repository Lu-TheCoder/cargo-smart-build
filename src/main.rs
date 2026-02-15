use anyhow::Result;
use clap::{Parser, Subcommand};

mod builder;
mod cargo_project;
mod config;
mod error;
mod features;
mod package_selector;
mod target_selector;
mod ui;

#[derive(Parser)]
#[command(name = "cargo-smart-build")]
#[command(bin_name = "cargo")]
enum CargoCli {
    SmartBuild(SmartBuildArgs),
}

#[derive(clap::Args)]
#[command(version, about, long_about = None)]
struct SmartBuildArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Clone, Copy, Debug, PartialEq)]
pub enum Commands {
    /// Build the selected package (default)
    Build,
    /// Run the selected package binary
    Run,
    /// Run tests for the selected package
    Test,
}

fn main() -> Result<()> {
    let CargoCli::SmartBuild(args) = CargoCli::parse();
    let command = args.command.unwrap_or(Commands::Build);

    //~ Load last config
    let last_config = config::load_config();

    let metadata = cargo_project::load_metadata()?;
    let default_package = last_config.as_ref().map(|config| config.package.clone());
    let package = package_selector::select_package(&metadata, default_package)?;

    let features = features::extract_features(package);

    //~ Use previous build choices as defaults if available
    let default_release = last_config.as_ref().map(|config| config.release);
    let default_target = last_config.as_ref().and_then(|config| config.target.clone());
    let default_features = last_config
        .as_ref()
        .map(|c| c.features.as_slice())
        .unwrap_or(&[]);

    let is_release = ui::select_build_mode(default_release)?;
    let target = target_selector::select_target(default_target)?;
    let selected_features = ui::select_features(&features, default_features)?;

    builder::execute_command(command, &package.name, is_release, target.as_deref(), &selected_features)?;

    //~ Save current selection
    let current_config = config::BuildConfig {
        package: package.name.to_string(),
        release: is_release,
        target,
        features: selected_features,
    };
    config::save_config(&current_config);

    Ok(())
}
