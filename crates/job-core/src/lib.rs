use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobState {
    Queued,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobProgress {
    pub current: u64,
    pub total: Option<u64>,
    pub message: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDescriptor {
    pub id: String,
    pub product_id: String,
    pub kind: String,
    pub state: JobState,
    pub progress: Option<JobProgress>,
    pub payload: Value,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfficeJobKind {
    Import,
    Export,
    Autosave,
    Recover,
    Thumbnail,
    Ai,
}

impl OfficeJobKind {
    pub fn as_str(self) -> &'static str {
        match self {
            OfficeJobKind::Import => "office.import",
            OfficeJobKind::Export => "office.export",
            OfficeJobKind::Autosave => "office.autosave",
            OfficeJobKind::Recover => "office.recover",
            OfficeJobKind::Thumbnail => "office.thumbnail",
            OfficeJobKind::Ai => "office.ai",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeJobPayload {
    pub artifact_id: Option<String>,
    pub product_id: String,
    pub office_kind: OfficeJobKind,
    pub cancellable: bool,
    #[serde(default)]
    pub warning_count: u32,
    #[serde(default)]
    pub details: Value,
}

pub fn office_job_descriptor(
    id: impl Into<String>,
    product_id: impl Into<String>,
    office_kind: OfficeJobKind,
    state: JobState,
    payload: OfficeJobPayload,
) -> JobDescriptor {
    JobDescriptor {
        id: id.into(),
        product_id: product_id.into(),
        kind: office_kind.as_str().to_string(),
        state,
        progress: None,
        payload: serde_json::to_value(payload).unwrap_or(Value::Null),
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobEvent {
    Created(JobDescriptor),
    Progress { id: String, progress: JobProgress },
    StateChanged { id: String, state: JobState },
    Log { id: String, message: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn office_job_descriptor_uses_stable_kind_string() {
        let payload = OfficeJobPayload {
            artifact_id: Some("deck_1".to_string()),
            product_id: "tench-slides".to_string(),
            office_kind: OfficeJobKind::Export,
            cancellable: true,
            warning_count: 2,
            details: json!({ "format": "pdf" }),
        };

        let descriptor = office_job_descriptor(
            "job_1",
            "tench-slides",
            OfficeJobKind::Export,
            JobState::Queued,
            payload,
        );

        assert_eq!(descriptor.kind, "office.export");
        assert_eq!(descriptor.payload["office_kind"], "export");
        assert_eq!(descriptor.payload["warning_count"], 2);
        assert_eq!(descriptor.payload["details"]["format"], "pdf");
    }

    // Edge case tests
    #[test]
    fn all_job_state_variants_roundtrip() {
        for variant in [
            JobState::Queued,
            JobState::Running,
            JobState::Paused,
            JobState::Completed,
            JobState::Failed,
            JobState::Cancelled,
        ] {
            let serialized = serde_json::to_string(&variant).unwrap();
            let deserialized: JobState = serde_json::from_str(&serialized).unwrap();
            assert_eq!(variant, deserialized);
        }
    }

    #[test]
    fn all_office_job_kind_variants_roundtrip() {
        for variant in [
            OfficeJobKind::Import,
            OfficeJobKind::Export,
            OfficeJobKind::Autosave,
            OfficeJobKind::Recover,
            OfficeJobKind::Thumbnail,
            OfficeJobKind::Ai,
        ] {
            let serialized = serde_json::to_string(&variant).unwrap();
            let deserialized: OfficeJobKind = serde_json::from_str(&serialized).unwrap();
            assert_eq!(variant, deserialized);
            assert!(!variant.as_str().is_empty());
        }
    }

    #[test]
    fn job_progress_zero_and_none() {
        let progress = JobProgress {
            current: 0,
            total: None,
            message: None,
        };
        let serialized = serde_json::to_string(&progress).unwrap();
        let deserialized: JobProgress = serde_json::from_str(&serialized).unwrap();
        assert_eq!(progress, deserialized);
    }

    #[test]
    fn job_progress_very_large_current() {
        let progress = JobProgress {
            current: u64::MAX,
            total: Some(u64::MAX),
            message: Some(String::new()),
        };
        let serialized = serde_json::to_string(&progress).unwrap();
        let deserialized: JobProgress = serde_json::from_str(&serialized).unwrap();
        assert_eq!(progress, deserialized);
    }

    #[test]
    fn job_descriptor_empty_strings_and_none() {
        let descriptor = JobDescriptor {
            id: String::new(),
            product_id: String::new(),
            kind: String::new(),
            state: JobState::Queued,
            progress: None,
            payload: json!({}),
        };
        let serialized = serde_json::to_string(&descriptor).unwrap();
        let deserialized: JobDescriptor = serde_json::from_str(&serialized).unwrap();
        assert_eq!(descriptor, deserialized);
    }

    #[test]
    fn office_job_payload_none_artifact_id() {
        let payload = OfficeJobPayload {
            artifact_id: None,
            product_id: String::new(),
            office_kind: OfficeJobKind::Ai,
            cancellable: false,
            warning_count: 0,
            details: json!({}),
        };
        let serialized = serde_json::to_string(&payload).unwrap();
        let deserialized: OfficeJobPayload = serde_json::from_str(&serialized).unwrap();
        assert_eq!(payload, deserialized);
    }

    #[test]
    fn office_job_payload_very_large_warning_count() {
        let payload = OfficeJobPayload {
            artifact_id: None,
            product_id: String::new(),
            office_kind: OfficeJobKind::Ai,
            cancellable: false,
            warning_count: u32::MAX,
            details: json!({}),
        };
        let serialized = serde_json::to_string(&payload).unwrap();
        let deserialized: OfficeJobPayload = serde_json::from_str(&serialized).unwrap();
        assert_eq!(payload, deserialized);
    }

    #[test]
    fn office_job_descriptor_with_empty_payload() {
        let payload = OfficeJobPayload {
            artifact_id: None,
            product_id: String::new(),
            office_kind: OfficeJobKind::Autosave,
            cancellable: false,
            warning_count: 0,
            details: json!({}),
        };
        let descriptor =
            office_job_descriptor("", "", OfficeJobKind::Autosave, JobState::Running, payload);
        assert_eq!(descriptor.id, "");
        assert_eq!(descriptor.product_id, "");
        assert_eq!(descriptor.kind, "office.autosave");
        assert_eq!(descriptor.state, JobState::Running);
        assert!(descriptor.progress.is_none());
    }

    #[test]
    fn job_event_created_roundtrip() {
        let descriptor = JobDescriptor {
            id: String::new(),
            product_id: String::new(),
            kind: String::new(),
            state: JobState::Queued,
            progress: None,
            payload: json!({}),
        };
        let event = JobEvent::Created(descriptor.clone());
        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: JobEvent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(event, deserialized);
    }

    #[test]
    fn job_event_progress_roundtrip() {
        let event = JobEvent::Progress {
            id: String::new(),
            progress: JobProgress {
                current: 0,
                total: None,
                message: None,
            },
        };
        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: JobEvent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(event, deserialized);
    }

    #[test]
    fn job_event_state_changed_roundtrip() {
        let event = JobEvent::StateChanged {
            id: String::new(),
            state: JobState::Failed,
        };
        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: JobEvent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(event, deserialized);
    }

    #[test]
    fn job_event_log_roundtrip() {
        let event = JobEvent::Log {
            id: String::new(),
            message: String::new(),
        };
        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: JobEvent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(event, deserialized);
    }
}
