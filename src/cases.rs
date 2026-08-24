use crate::CaseStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    Standard,
    Extended,
    LegalHoldEligible,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptedObjectReference {
    /// Opaque object-store reference; never a URL, bearer credential, or document bytes.
    pub object_reference: String,
    /// Lowercase SHA-256 hex of the encrypted object.
    pub ciphertext_sha256: String,
    pub key_version: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCaseCommand {
    pub title: String,
    #[serde(default)]
    pub summary: String,
    pub client_reference: String,
    pub destination_country: String,
    pub document_type: String,
    pub next_action_due_at: Option<DateTime<Utc>>,
    pub retention_class: RetentionClass,
    pub retain_until: DateTime<Utc>,
    pub encrypted_document: Option<EncryptedObjectReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedCase {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub version: i64,
    pub title: String,
    pub summary: String,
    pub client_reference: String,
    pub destination_country: String,
    pub document_type: String,
    pub next_action_due_at: Option<DateTime<Utc>>,
    pub status: CaseStatus,
    pub retention_class: RetentionClass,
    pub retain_until: DateTime<Utc>,
    pub legal_hold: bool,
    pub tombstoned_at: Option<DateTime<Utc>>,
    pub encrypted_document: Option<EncryptedObjectReference>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionCaseCommand {
    pub expected_version: i64,
    pub to_status: CaseStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseEventReceipt {
    pub event_id: Uuid,
    pub tenant_id: Uuid,
    pub case_id: Uuid,
    pub case_version: i64,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    pub event_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseMutationResult {
    pub case: PersistedCase,
    pub event: CaseEventReceipt,
    pub replayed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaseConflictCode {
    IdempotencyKeyReused,
    StaleVersion,
    InvalidTransition,
}

impl CreateCaseCommand {
    pub fn validate(&self, now: DateTime<Utc>) -> Result<(), String> {
        if self.title.trim().is_empty() || self.title.len() > 256 {
            return Err("title must contain 1 through 256 bytes".into());
        }
        if self.summary.len() > 4_000 || self.client_reference.trim().is_empty() {
            return Err("invalid sanitized case metadata".into());
        }
        if self.destination_country.trim().is_empty() || self.document_type.trim().is_empty() {
            return Err("destination_country and document_type are required".into());
        }
        if self.retain_until <= now {
            return Err("retain_until must be in the future".into());
        }
        if let Some(document) = &self.encrypted_document {
            if document.object_reference.trim().is_empty()
                || document.object_reference.contains("://")
                || document.ciphertext_sha256.len() != 64
                || !document
                    .ciphertext_sha256
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
                || document.key_version <= 0
            {
                return Err("invalid encrypted document reference".into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn command() -> CreateCaseCommand {
        CreateCaseCommand {
            title: "Apostille intake".into(),
            summary: "Sanitized operational metadata".into(),
            client_reference: "CLIENT-42".into(),
            destination_country: "Peru".into(),
            document_type: "birth_certificate".into(),
            next_action_due_at: None,
            retention_class: RetentionClass::Standard,
            retain_until: Utc.with_ymd_and_hms(2030, 1, 1, 0, 0, 0).unwrap(),
            encrypted_document: Some(EncryptedObjectReference {
                object_reference: "objects/tenant-42/ciphertext-7".into(),
                ciphertext_sha256:
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
                key_version: 7,
            }),
        }
    }

    #[test]
    fn encrypted_reference_accepts_only_opaque_ciphertext_metadata() {
        let now = Utc.with_ymd_and_hms(2026, 8, 24, 0, 0, 0).unwrap();
        assert!(command().validate(now).is_ok());

        let mut exposed_url = command();
        exposed_url
            .encrypted_document
            .as_mut()
            .unwrap()
            .object_reference = "https://object-store.example/document".into();
        assert!(exposed_url.validate(now).is_err());

        let mut uppercase_digest = command();
        uppercase_digest
            .encrypted_document
            .as_mut()
            .unwrap()
            .ciphertext_sha256 =
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".into();
        assert!(uppercase_digest.validate(now).is_err());
    }

    #[test]
    fn mutation_contract_requires_an_explicit_expected_version() {
        let command = TransitionCaseCommand {
            expected_version: 4,
            to_status: CaseStatus::Submitted,
        };
        assert_eq!(command.expected_version, 4);
        assert_eq!(command.to_status, CaseStatus::Submitted);
    }
}
