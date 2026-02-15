use anyhow::Result;
use cargo_metadata::{Metadata, Package};
use dialoguer::Select;

pub fn select_package<'a>(metadata: &'a Metadata) -> Result<&'a Package> {
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
    let names: Vec<&str> = packages
        .iter()
        .map(|package| package.name.as_str())
        .collect();

    let selection = Select::new()
        .with_prompt("Select package")
        .items(&names)
        .default(0)
        .interact()?;

    Ok(packages[selection])
}
