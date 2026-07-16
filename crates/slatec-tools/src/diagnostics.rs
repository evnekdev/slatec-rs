use crate::hash;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StageStatus {
    Success,
    SuccessWithReviewItems,
    Failed,
}

#[derive(Clone, Debug, Serialize)]
pub struct Diagnostic {
    pub id: String,
    pub rule_id: String,
    pub severity: String,
    pub stage: String,
    pub message_template_id: String,
    pub arguments: Vec<String>,
    pub review_impact: String,
}

impl Diagnostic {
    pub fn new(rule_id: &str, severity: &str, stage: &str, arguments: Vec<String>) -> Self {
        let joined = format!(
            "{rule_id}\u{1f}{severity}\u{1f}{stage}\u{1f}{}",
            arguments.join("\u{1f}")
        );
        Self {
            id: format!("diag-{}", &hash::bytes(joined.as_bytes())[..16]),
            rule_id: rule_id.to_owned(),
            severity: severity.to_owned(),
            stage: stage.to_owned(),
            message_template_id: rule_id.to_owned(),
            arguments,
            review_impact: if severity == "error" {
                "blocking"
            } else {
                "review"
            }
            .to_owned(),
        }
    }
}
