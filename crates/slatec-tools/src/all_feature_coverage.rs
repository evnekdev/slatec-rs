//! Deterministic validation for the public `slatec-sys::all` feature.
//!
//! The maintained registry is deliberately the authority for what counts as a
//! public mathematical family. Cargo feature spelling alone is insufficient:
//! raw ABI batches, profile gates, and test-only switches must never silently
//! become part of the public aggregate.

use crate::error::{CorpusError, Result};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Registry {
    schema_id: String,
    schema_version: String,
    public_family_aggregates: Vec<String>,
    feature_classes: BTreeMap<String, Vec<String>>,
}

/// Generates the machine-readable public-family coverage report.
pub fn generate(sys_cargo_toml: &Path, registry_path: &Path) -> Result<Value> {
    let registry: Registry = serde_json::from_slice(&fs::read(registry_path)?)?;
    if registry.schema_id != "slatec-sys.public-family-feature-registry"
        || registry.schema_version != "1.0.0"
    {
        return Err(policy("unsupported public-family feature registry"));
    }
    let features = cargo_features(sys_cargo_toml)?;
    let direct = features
        .get("all")
        .ok_or_else(|| policy("slatec-sys must define the public all feature"))?
        .clone();
    let closure = closure(&features, "all")?;
    let public = registry
        .public_family_aggregates
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let covered = public
        .iter()
        .filter(|feature| closure.contains(*feature))
        .cloned()
        .collect::<Vec<_>>();
    let missing = public
        .iter()
        .filter(|feature| !closure.contains(*feature))
        .cloned()
        .collect::<Vec<_>>();
    let direct_set = direct.iter().cloned().collect::<BTreeSet<_>>();
    let unexpected_direct = direct_set.difference(&public).cloned().collect::<Vec<_>>();
    let absent_direct = public.difference(&direct_set).cloned().collect::<Vec<_>>();
    let mut classified = BTreeSet::from(["all".to_owned()]);
    classified.extend(public.iter().cloned());
    for items in registry.feature_classes.values() {
        for item in items {
            if !features.contains_key(item) {
                return Err(policy(&format!(
                    "public-family feature registry names missing slatec-sys feature {item}"
                )));
            }
            classified.insert(item.clone());
        }
    }
    let class_report = registry
        .feature_classes
        .iter()
        .map(|(class, members)| {
            let rows = members
                .iter()
                .map(|feature| {
                    json!({
                        "feature":feature,
                        "direct_member_of_all":direct_set.contains(feature),
                        "transitively_reachable_from_all":closure.contains(feature),
                    })
                })
                .collect::<Vec<_>>();
            (class.clone(), Value::Array(rows))
        })
        .collect::<BTreeMap<_, _>>();
    let public_components = closure
        .iter()
        .filter(|feature| !classified.contains(*feature))
        .cloned()
        .collect::<Vec<_>>();
    let report = json!({
        "schema_id":"slatec-sys.raw-api.all-feature-coverage",
        "schema_version":"1.0.0",
        "policy":"all directly names every authored public mathematical family aggregate. It never directly names provider, profile-only, raw ABI-shape, raw-family, or test-only implementation features; legitimate public family dependencies remain visible in the transitive closure.",
        "public_family_aggregates":public,
        "families_covered_by_all":covered,
        "families_missing_from_all":missing,
        "all_direct_members":direct,
        "unexpected_direct_members":unexpected_direct,
        "public_family_members_missing_directly":absent_direct,
        "transitive_closure":closure,
        "public_subfamilies_or_components":public_components,
        "intentionally_excluded_feature_classes":class_report,
        "provider_features_intentionally_excluded":["bundled", "source-build", "system", "external-backend"],
    });
    if !report["families_missing_from_all"]
        .as_array()
        .is_some_and(|items| items.is_empty())
        || !report["unexpected_direct_members"]
            .as_array()
            .is_some_and(|items| items.is_empty())
        || !report["public_family_members_missing_directly"]
            .as_array()
            .is_some_and(|items| items.is_empty())
    {
        return Err(policy(
            "slatec-sys all does not exactly cover the authored public-family registry",
        ));
    }
    Ok(report)
}

fn cargo_features(path: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    let text = fs::read_to_string(path)?;
    let value: toml::Value = toml::from_str(&text)?;
    value
        .get("features")
        .and_then(toml::Value::as_table)
        .ok_or_else(|| policy("Cargo.toml has no features table"))?
        .iter()
        .map(|(name, value)| {
            let items = value
                .as_array()
                .ok_or_else(|| policy("Cargo feature must be an array"))?
                .iter()
                .filter_map(toml::Value::as_str)
                .map(|item| item.strip_prefix("dep:").unwrap_or(item).to_owned())
                .collect();
            Ok((name.clone(), items))
        })
        .collect()
}

fn closure(features: &BTreeMap<String, Vec<String>>, root: &str) -> Result<BTreeSet<String>> {
    let mut result = BTreeSet::new();
    let mut pending = vec![root.to_owned()];
    while let Some(feature) = pending.pop() {
        if !result.insert(feature.clone()) {
            continue;
        }
        let dependencies = features
            .get(&feature)
            .ok_or_else(|| policy(&format!("feature {feature} names an unknown dependency")))?;
        pending.extend(dependencies.iter().cloned());
    }
    Ok(result)
}

fn policy(message: &str) -> CorpusError {
    CorpusError::Verification(format!("all-feature coverage policy: {message}"))
}
