use cargo_metadata::Package;

pub fn extract_features(package: &Package) -> Vec<String> {
    let mut features: Vec<String> = package
        .features
        .keys()
        .filter(|f| *f != "default")
        .cloned()
        .collect();

    features.sort();
    features
}
