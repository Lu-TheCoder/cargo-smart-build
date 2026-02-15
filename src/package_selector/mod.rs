use anyhow::Result;
use cargo_metadata::{Metadata, Package};
use dialoguer::Select;
use console::style;

pub fn select_package<'a>(metadata: &'a Metadata, default_package: Option<String>) -> Result<&'a Package> {
    let workspace_members = &metadata.workspace_members;

    //~ Collect workspace packages
    let packages: Vec<&Package> = metadata
        .packages
        .iter()
        .filter(|package| workspace_members.contains(&package.id))
        .collect();

    if packages.len() == 1 {
        return Ok(packages[0]);
    }

    //~ If we detect Multiple packages then we prompt user:
    let names: Vec<String> = packages
        .iter()
        .map(|package| {
            let name = package.name.as_str();
            if let Some(default) = &default_package {
                if name == default {
                    return format!("{}", style(name).green().bold());
                }
            }
            name.to_string()
        })
        .collect();

    let default_index = if let Some(default) = &default_package {
        packages.iter().position(|p| p.name == *default).unwrap_or(0)
    } else {
        0
    };

    let selection = Select::new()
        .with_prompt("Select package")
        .items(&names)
        .default(default_index)
        .interact()?;

    Ok(packages[selection])
}
