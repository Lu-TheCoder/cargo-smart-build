use anyhow::{Context, Result};
use cargo_metadata::{Metadata, MetadataCommand, Package};

pub fn load_metadata() -> Result<Metadata> {
    MetadataCommand::new()
        .exec()
        .context("Not inside a Cargo project. Could not find Cargo.toml")
}

pub fn root_package(metadata: &Metadata) -> Result<&Package> {
    metadata.root_package().context("No root package found")
}
