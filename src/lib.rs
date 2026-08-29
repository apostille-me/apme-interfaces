pub mod cases;
pub mod security;

pub const TENANT_CASE_PERSISTENCE_MIGRATION: &str =
    include_str!("../sql/002_tenant_case_persistence.sql");

    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use std::fmt;
    use uuid::Uuid;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum CaseStatus {
        Intake,
CollectingDocuments,
Review,
Submitted,
Completed,
Closed
    }

    impl Default for CaseStatus {
        fn default() -> Self { Self::Intake }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Case {
        pub id: Uuid,
        pub title: String,
        pub summary: String,
        pub client_reference: String,
pub destination_country: String,
pub document_type: String,
pub next_action_due_at: Option<DateTime<Utc>>,
        pub status: CaseStatus,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateCase {
        pub title: String,
        #[serde(default)]
        pub summary: String,
        pub client_reference: String,
pub destination_country: String,
pub document_type: String,
pub next_action_due_at: Option<DateTime<Utc>>,
    }

    impl CreateCase {
        pub fn validate(&self) -> Result<(), ValidationError> {
            if self.title.trim().is_empty() { return Err(ValidationError("title must not be empty".into())); }
    if self.summary.len() > 4_000 { return Err(ValidationError("summary exceeds 4000 bytes".into())); }
    if self.client_reference.trim().is_empty() { return Err(ValidationError("client_reference must not be empty".into())); }
    if self.destination_country.trim().is_empty() { return Err(ValidationError("destination_country must not be empty".into())); }
    if self.document_type.trim().is_empty() { return Err(ValidationError("document_type must not be empty".into())); }
            Ok(())
        }

        pub fn into_record(self, id: Uuid, now: DateTime<Utc>) -> Result<Case, ValidationError> {
            self.validate()?;
            Ok(Case {
                id,
                title: self.title,
                summary: self.summary,
                client_reference: self.client_reference,
        destination_country: self.destination_country,
        document_type: self.document_type,
        next_action_due_at: self.next_action_due_at,
                status: CaseStatus::default(),
                created_at: now,
                updated_at: now,
            })
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CaseEvent {
        pub event_id: Uuid,
        pub event_type: String,
        pub occurred_at: DateTime<Utc>,
        pub data: Case,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ValidationError(pub String);

    impl fmt::Display for ValidationError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.0) }
    }

    impl std::error::Error for ValidationError {}

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn status_serializes_as_wire_value() {
            let value = serde_json::to_string(&CaseStatus::default()).unwrap();
            assert_eq!(value, serde_json::to_string(&"intake").unwrap());
        }
    }
