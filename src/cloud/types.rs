use serde::{Deserialize, Serialize};

/// Standard API response wrapper
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub result: Option<T>,
    #[allow(dead_code)]
    pub error: Option<ApiError>,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    #[allow(dead_code)]
    pub code: Option<String>,
    pub message: String,
}

/// Organization
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub created_at: Option<String>,
}

/// Service (ClickHouse Cloud instance)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub region: String,
    pub state: String,
    pub tier: Option<String>,
    pub idle_scaling: Option<bool>,
    pub idle_timeout_minutes: Option<u32>,
    pub ip_access_list: Option<Vec<IpAccessEntry>>,
    pub created_at: Option<String>,
    pub endpoints: Option<Vec<Endpoint>>,
    pub min_replica_memory_gb: Option<u32>,
    pub max_replica_memory_gb: Option<u32>,
    pub num_replicas: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IpAccessEntry {
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    pub protocol: String,
    pub host: String,
    pub port: u16,
}

/// Backup
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Backup {
    pub id: String,
    pub service_id: Option<String>,
    pub status: String,
    pub created_at: Option<String>,
    pub finished_at: Option<String>,
    pub size_in_bytes: Option<u64>,
}

/// Resource tag
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceTag {
    pub key: String,
    pub value: String,
}

/// Create service request - all non-deprecated fields from OpenAPI spec
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceRequest {
    /// Name of the service (required)
    pub name: String,

    /// Cloud provider: aws, gcp, azure (required)
    pub provider: String,

    /// Service region (required)
    pub region: String,

    /// List of IP addresses allowed to access the service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_access_list: Option<Vec<IpAccessEntry>>,

    /// Minimum memory per replica in GB (8-356, multiple of 4)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_replica_memory_gb: Option<u32>,

    /// Maximum memory per replica in GB (8-356, multiple of 4)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_replica_memory_gb: Option<u32>,

    /// Number of replicas (1-20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_replicas: Option<u32>,

    /// Allow scale to zero when idle (default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_scaling: Option<bool>,

    /// Minimum idle timeout in minutes (>= 5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_minutes: Option<u32>,

    /// Backup ID to restore from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,

    /// Release channel: slow, default, fast
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_channel: Option<String>,

    /// Tags for the service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ResourceTag>>,

    /// Data warehouse ID (for read replicas)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_warehouse_id: Option<String>,

    /// Make service read-only (requires data_warehouse_id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_readonly: Option<bool>,

    /// Customer-provided disk encryption key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,

    /// Role for disk encryption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_assumed_role_identifier: Option<String>,

    /// Enable Transparent Data Encryption (enterprise only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_transparent_data_encryption: Option<bool>,

    /// BYOC region ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byoc_id: Option<String>,

    /// Compliance type: hipaa, pci
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,

    /// Custom instance profile (enterprise only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,

    /// Accept private preview terms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_preview_terms_checked: Option<bool>,
}

/// Create service response includes credentials
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceResponse {
    pub service: Service,
    pub password: String,
}

/// State change request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StateChangeRequest {
    pub command: String, // "start" or "stop"
}
