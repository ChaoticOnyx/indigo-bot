use super::{Secret, ServiceId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    pub name: String,
    pub secret: Secret,
    pub service_id: ServiceId,
    pub created_at: DateTime<Utc>,
    pub configuration: WebhookConfiguration,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WebhookConfiguration(pub serde_json::Value);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WebhookPayload(pub serde_json::Value);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WebhookResponse(pub serde_json::Value);

impl From<serde_json::Value> for WebhookPayload {
    fn from(payload: serde_json::Value) -> Self {
        serde_json::from_value(payload).unwrap()
    }
}

impl From<serde_json::Value> for WebhookResponse {
    fn from(response: serde_json::Value) -> Self {
        serde_json::from_value(response).unwrap()
    }
}
