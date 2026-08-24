use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TenantRole {
    Viewer,
    Agent,
    Administrator,
}

impl TenantRole {
    pub fn can_create_or_transition(self) -> bool {
        matches!(self, Self::Agent | Self::Administrator)
    }

    pub fn can_export_or_delete(self) -> bool {
        matches!(self, Self::Administrator)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifiedSubject {
    /// Stable Shared Auth `sub`; product authorization is loaded separately.
    pub shared_user_id: String,
    pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TenantContext {
    pub tenant_id: Uuid,
    pub organization_id: Uuid,
    pub role: TenantRole,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseSubscription {
    pub tenant_id: Uuid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case_id: Option<Uuid>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_roles_keep_mutation_and_export_authority_distinct() {
        assert!(!TenantRole::Viewer.can_create_or_transition());
        assert!(TenantRole::Agent.can_create_or_transition());
        assert!(!TenantRole::Agent.can_export_or_delete());
        assert!(TenantRole::Administrator.can_create_or_transition());
        assert!(TenantRole::Administrator.can_export_or_delete());
    }
}
