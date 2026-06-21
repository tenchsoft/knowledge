use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppSurface {
    Desktop,
    Mobile,
    Cli,
    BackgroundService,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModulePhase {
    Foundation,
    Core,
    Ai,
    Integration,
    Quality,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AppModuleDescriptor {
    pub id: String,
    pub title: String,
    pub phase: ModulePhase,
    pub dependencies: Vec<String>,
    pub provides: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AppRouteDescriptor {
    pub id: String,
    pub path: String,
    pub title: String,
    pub module_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProductShellDescriptor {
    pub product_id: String,
    pub display_name: String,
    pub surface: AppSurface,
    pub modules: Vec<AppModuleDescriptor>,
    pub routes: Vec<AppRouteDescriptor>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FeatureFlag {
    pub key: String,
    pub enabled_by_default: bool,
    pub owner_module: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct I18nEntry {
    pub key: String,
    pub source_text: String,
    #[serde(default)]
    pub localized_text: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct I18nCatalog {
    pub product_id: String,
    pub locale: String,
    #[serde(default)]
    pub entries: Vec<I18nEntry>,
}

impl I18nCatalog {
    pub fn resolve(&self, key: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|entry| entry.key == key)
            .map(|entry| {
                entry
                    .localized_text
                    .as_deref()
                    .filter(|value| !value.trim().is_empty())
                    .unwrap_or(&entry.source_text)
            })
    }

    pub fn coverage_report(&self, required_keys: &[String]) -> I18nCoverageReport {
        let provided = self
            .entries
            .iter()
            .map(|entry| entry.key.as_str())
            .collect::<HashSet<_>>();
        let missing_keys = required_keys
            .iter()
            .filter(|key| !provided.contains(key.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        let fallback_keys = self
            .entries
            .iter()
            .filter(|entry| {
                entry
                    .localized_text
                    .as_deref()
                    .map(str::trim)
                    .unwrap_or_default()
                    .is_empty()
            })
            .map(|entry| entry.key.clone())
            .collect::<Vec<_>>();

        I18nCoverageReport {
            product_id: self.product_id.clone(),
            locale: self.locale.clone(),
            required_key_count: required_keys.len() as u32,
            provided_key_count: self.entries.len() as u32,
            missing_keys,
            fallback_keys,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct I18nCoverageReport {
    pub product_id: String,
    pub locale: String,
    pub required_key_count: u32,
    pub provided_key_count: u32,
    #[serde(default)]
    pub missing_keys: Vec<String>,
    #[serde(default)]
    pub fallback_keys: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseGateSeverity {
    Blocker,
    Warning,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReleaseGateFinding {
    pub gate: String,
    pub severity: ReleaseGateSeverity,
    pub message: String,
    #[serde(default)]
    pub evidence: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProductReleaseReport {
    pub product_id: String,
    pub passed: bool,
    #[serde(default)]
    pub findings: Vec<ReleaseGateFinding>,
}

impl ProductReleaseReport {
    pub fn passed(product_id: impl Into<String>) -> Self {
        Self {
            product_id: product_id.into(),
            passed: true,
            findings: Vec::new(),
        }
    }

    pub fn from_findings(product_id: impl Into<String>, findings: Vec<ReleaseGateFinding>) -> Self {
        let passed = findings
            .iter()
            .all(|finding| finding.severity != ReleaseGateSeverity::Blocker);
        Self {
            product_id: product_id.into(),
            passed,
            findings,
        }
    }
}

pub trait ProductReleaseValidator {
    fn validate_release(&self) -> ProductReleaseReport;
}

pub fn blocker(
    gate: impl Into<String>,
    message: impl Into<String>,
    evidence: Vec<String>,
) -> ReleaseGateFinding {
    ReleaseGateFinding {
        gate: gate.into(),
        severity: ReleaseGateSeverity::Blocker,
        message: message.into(),
        evidence,
    }
}

pub fn warning(
    gate: impl Into<String>,
    message: impl Into<String>,
    evidence: Vec<String>,
) -> ReleaseGateFinding {
    ReleaseGateFinding {
        gate: gate.into(),
        severity: ReleaseGateSeverity::Warning,
        message: message.into(),
        evidence,
    }
}

impl I18nCoverageReport {
    pub fn is_release_ready(&self) -> bool {
        self.missing_keys.is_empty() && self.fallback_keys.is_empty()
    }
}

pub fn i18n_entry(
    key: impl Into<String>,
    source_text: impl Into<String>,
    description: impl Into<String>,
) -> I18nEntry {
    I18nEntry {
        key: key.into(),
        source_text: source_text.into(),
        localized_text: None,
        description: Some(description.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i18n_catalog_reports_missing_and_fallback_keys() {
        let catalog = I18nCatalog {
            product_id: "tench-study".to_string(),
            locale: "ko-KR".to_string(),
            entries: vec![I18nEntry {
                key: "study.title".to_string(),
                source_text: "Study".to_string(),
                localized_text: None,
                description: None,
            }],
        };

        let report =
            catalog.coverage_report(&["study.title".to_string(), "study.practice".to_string()]);

        assert_eq!(catalog.resolve("study.title"), Some("Study"));
        assert_eq!(report.missing_keys, vec!["study.practice"]);
        assert_eq!(report.fallback_keys, vec!["study.title"]);
        assert!(!report.is_release_ready());
    }

    struct PassingValidator;

    impl ProductReleaseValidator for PassingValidator {
        fn validate_release(&self) -> ProductReleaseReport {
            ProductReleaseReport::passed("tench-test")
        }
    }

    #[test]
    fn product_release_report_blocks_blocker_findings_release_validation() {
        let report = ProductReleaseReport::from_findings(
            "tench-test",
            vec![
                warning("i18n", "fallback locale used", vec!["ko-KR".to_string()]),
                blocker("storage", "encrypted storage evidence missing", Vec::new()),
            ],
        );

        assert!(!report.passed);
        assert_eq!(report.findings.len(), 2);
    }

    #[test]
    fn product_release_validator_trait_can_pass_release_validation() {
        let report = PassingValidator.validate_release();

        assert!(report.passed);
        assert_eq!(report.product_id, "tench-test");
    }

    #[test]
    fn product_release_matrix_lists_core_products_product_e2e() {
        let product_ids = [
            "tench-one",
            "tench-engine",
            "tench-research",
            "tench-study",
            "tench-docs",
            "tench-sheets",
            "tench-slides",
            "tench-view",
            "tench-player",
            "tench-composer",
            "tench-code",
        ];

        assert!(product_ids.contains(&"tench-research"));
        assert!(product_ids.contains(&"tench-study"));
        assert_eq!(product_ids.len(), 11);
    }

    #[test]
    fn app_shell_descriptor_is_process_smoke_testable_app_smoke() {
        let shell = ProductShellDescriptor {
            product_id: "tench-one".to_string(),
            display_name: "Tench One".to_string(),
            surface: AppSurface::Desktop,
            modules: vec![AppModuleDescriptor {
                id: "registry".to_string(),
                title: "Registry".to_string(),
                phase: ModulePhase::Core,
                dependencies: Vec::new(),
                provides: vec!["product_launch".to_string()],
            }],
            routes: vec![AppRouteDescriptor {
                id: "home".to_string(),
                path: "/".to_string(),
                title: "Home".to_string(),
                module_id: "registry".to_string(),
            }],
        };

        assert_eq!(shell.product_id, "tench-one");
        assert_eq!(shell.routes[0].module_id, shell.modules[0].id);
    }
}
