//! Admin namespace for super-admin API endpoints.

use crate::{Error, KuberNodes, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;

/// Entry point for all admin sub-resources.
pub struct AdminService<'a> {
    client: &'a KuberNodes,
}

impl<'a> AdminService<'a> {
    pub(crate) fn new(client: &'a KuberNodes) -> Self {
        Self { client }
    }

    pub fn smtp(&self) -> SmtpResource<'_> { SmtpResource { client: self.client } }
    pub fn payment_gateways(&self) -> PaymentGatewaysResource<'_> { PaymentGatewaysResource { client: self.client } }
    pub fn storage(&self) -> StorageResource<'_> { StorageResource { client: self.client } }
    pub fn domains(&self) -> DomainsResource<'_> { DomainsResource { client: self.client } }
    pub fn rate_limits(&self) -> RateLimitsResource<'_> { RateLimitsResource { client: self.client } }
    pub fn webhooks(&self) -> WebhooksResource<'_> { WebhooksResource { client: self.client } }
    pub fn email_templates(&self) -> EmailTemplatesResource<'_> { EmailTemplatesResource { client: self.client } }
    pub fn ip_rules(&self) -> IpRulesResource<'_> { IpRulesResource { client: self.client } }
    pub fn feature_flags(&self) -> FeatureFlagsResource<'_> { FeatureFlagsResource { client: self.client } }
    pub fn audit(&self) -> AuditResource<'_> { AuditResource { client: self.client } }
    pub fn health(&self) -> HealthResource<'_> { HealthResource { client: self.client } }
    pub fn maintenance(&self) -> MaintenanceResource<'_> { MaintenanceResource { client: self.client } }
    pub fn sso(&self) -> SsoResource<'_> { SsoResource { client: self.client } }
    pub fn backups(&self) -> BackupsResource<'_> { BackupsResource { client: self.client } }
}

// ── Internal HTTP helpers ────────────────────────────────────────────────────

fn admin_headers(client: &KuberNodes) -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    h.insert("X-API-Key", HeaderValue::from_str(&client.api_key).unwrap());
    if let Some(ref tok) = client.admin_token {
        if !tok.is_empty() {
            h.insert("X-Admin-Token", HeaderValue::from_str(tok).unwrap());
        }
    }
    let bearer = match &client.access_token {
        Some(t) if !t.is_empty() => format!("Bearer {t}"),
        _ => format!("Bearer {}", client.api_key),
    };
    h.insert(AUTHORIZATION, HeaderValue::from_str(&bearer).unwrap());
    h
}

async fn admin_get(client: &KuberNodes, path: &str) -> Result<Value> {
    let url = format!("{}{}", client.base_url, path);
    let resp = client.http.get(&url).headers(admin_headers(client)).send().await?;
    parse_response(resp).await
}

async fn admin_post(client: &KuberNodes, path: &str, body: Option<Value>) -> Result<Value> {
    let url = format!("{}{}", client.base_url, path);
    let mut req = client.http.post(&url).headers(admin_headers(client));
    if let Some(b) = body {
        req = req.json(&b);
    }
    let resp = req.send().await?;
    parse_response(resp).await
}

async fn admin_put(client: &KuberNodes, path: &str, body: Value) -> Result<Value> {
    let url = format!("{}{}", client.base_url, path);
    let resp = client.http.put(&url).headers(admin_headers(client)).json(&body).send().await?;
    parse_response(resp).await
}

async fn admin_delete(client: &KuberNodes, path: &str) -> Result<Value> {
    let url = format!("{}{}", client.base_url, path);
    let resp = client.http.delete(&url).headers(admin_headers(client)).send().await?;
    parse_response(resp).await
}

async fn parse_response(resp: reqwest::Response) -> Result<Value> {
    let status = resp.status().as_u16();
    let body = resp.text().await.unwrap_or_default();
    if status >= 400 {
        return Err(Error::Http { status, body });
    }
    if body.is_empty() {
        return Ok(Value::Null);
    }
    serde_json::from_str(&body).map_err(Into::into)
}

// ── SmtpResource ─────────────────────────────────────────────────────────────

pub struct SmtpResource<'a> { client: &'a KuberNodes }
impl SmtpResource<'_> {
    pub async fn list(&self) -> Result<Value> { admin_get(self.client, "/sa/settings/smtp").await }
    pub async fn get(&self, id: &str) -> Result<Value> { admin_get(self.client, &format!("/sa/settings/smtp/{id}")).await }
    pub async fn create(&self, data: Value) -> Result<Value> { admin_post(self.client, "/sa/settings/smtp", Some(data)).await }
    pub async fn update(&self, id: &str, data: Value) -> Result<Value> { admin_put(self.client, &format!("/sa/settings/smtp/{id}"), data).await }
    pub async fn delete(&self, id: &str) -> Result<Value> { admin_delete(self.client, &format!("/sa/settings/smtp/{id}")).await }
    pub async fn test(&self, id: &str) -> Result<Value> { admin_post(self.client, &format!("/sa/settings/smtp/{id}/test"), None).await }
    pub async fn list_changes(&self, hours: u32, limit: u32) -> Result<Value> { admin_get(self.client, &format!("/sa/settings/smtp/changes?hours={hours}&limit={limit}")).await }
}

// ── PaymentGatewaysResource ──────────────────────────────────────────────────

pub struct PaymentGatewaysResource<'a> { client: &'a KuberNodes }
impl PaymentGatewaysResource<'_> {
    pub async fn list(&self) -> Result<Value> { admin_get(self.client, "/sa/settings/payment-gateways").await }
    pub async fn get(&self, id: &str) -> Result<Value> { admin_get(self.client, &format!("/sa/settings/payment-gateways/{id}")).await }
    pub async fn create(&self, data: Value) -> Result<Value> { admin_post(self.client, "/sa/settings/payment-gateways", Some(data)).await }
    pub async fn update(&self, id: &str, data: Value) -> Result<Value> { admin_put(self.client, &format!("/sa/settings/payment-gateways/{id}"), data).await }
    pub async fn delete(&self, id: &str) -> Result<Value> { admin_delete(self.client, &format!("/sa/settings/payment-gateways/{id}")).await }
    pub async fn set_default(&self, id: &str) -> Result<Value> { admin_post(self.client, &format!("/sa/settings/payment-gateways/{id}/set-default"), None).await }
    pub async fn test(&self, id: &str) -> Result<Value> { admin_post(self.client, &format!("/sa/settings/payment-gateways/{id}/test"), None).await }
    pub async fn list_changes(&self, hours: u32, limit: u32) -> Result<Value> { admin_get(self.client, &format!("/sa/settings/payment-gateways/changes?hours={hours}&limit={limit}")).await }
}

// ── StorageResource ──────────────────────────────────────────────────────────

pub struct StorageResource<'a> { client: &'a KuberNodes }
impl StorageResource<'_> {
    pub async fn list(&self) -> Result<Value> { admin_get(self.client, "/sa/settings/storage").await }
    pub async fn get(&self, id: &str) -> Result<Value> { admin_get(self.client, &format!("/sa/settings/storage/{id}")).await }
    pub async fn create(&self, data: Value) -> Result<Value> { admin_post(self.client, "/sa/settings/storage", Some(data)).await }
    pub async fn update(&self, id: &str, data: Value) -> Result<Value> { admin_put(self.client, &format!("/sa/settings/storage/{id}"), data).await }
    pub async fn delete(&self, id: &str) -> Result<Value> { admin_delete(self.client, &format!("/sa/settings/storage/{id}")).await }
    pub async fn set_default(&self, id: &str) -> Result<Value> { admin_post(self.client, &format!("/sa/settings/storage/{id}/set-default"), None).await }
    pub async fn test(&self, id: &str) -> Result<Value> { admin_post(self.client, &format!("/sa/settings/storage/{id}/test"), None).await }
    pub async fn list_changes(&self, hours: u32, limit: u32) -> Result<Value> { admin_get(self.client, &format!("/sa/settings/storage/changes?hours={hours}&limit={limit}")).await }
}

// ── DomainsResource ──────────────────────────────────────────────────────────

pub struct DomainsResource<'a> { client: &'a KuberNodes }
impl DomainsResource<'_> {
    pub async fn list(&self) -> Result<Value> { admin_get(self.client, "/sa/settings/domains").await }
    pub async fn get(&self, id: &str) -> Result<Value> { admin_get(self.client, &format!("/sa/settings/domains/{id}")).await }
    pub async fn create(&self, data: Value) -> Result<Value> { admin_post(self.client, "/sa/settings/domains", Some(data)).await }
    pub async fn update(&self, id: &str, data: Value) -> Result<Value> { admin_put(self.client, &format!("/sa/settings/domains/{id}"), data).await }
    pub async fn delete(&self, id: &str) -> Result<Value> { admin_delete(self.client, &format!("/sa/settings/domains/{id}")).await }
    pub async fn verify(&self, id: &str) -> Result<Value> { admin_post(self.client, &format!("/sa/settings/domains/{id}/verify"), None).await }
    pub async fn list_changes(&self, hours: u32, limit: u32) -> Result<Value> { admin_get(self.client, &format!("/sa/settings/domains/changes?hours={hours}&limit={limit}")).await }
}

// ── RateLimitsResource ───────────────────────────────────────────────────────

pub struct RateLimitsResource<'a> { client: &'a KuberNodes }
impl RateLimitsResource<'_> {
    pub async fn list(&self) -> Result<Value> { admin_get(self.client, "/sa/settings/rate-limits").await }
    pub async fn get(&self, id: &str) -> Result<Value> { admin_get(self.client, &format!("/sa/settings/rate-limits/{id}")).await }
    pub async fn create(&self, data: Value) -> Result<Value> { admin_post(self.client, "/sa/settings/rate-limits", Some(data)).await }
    pub async fn update(&self, id: &str, data: Value) -> Result<Value> { admin_put(self.client, &format!("/sa/settings/rate-limits/{id}"), data).await }
    pub async fn delete(&self, id: &str) -> Result<Value> { admin_delete(self.client, &format!("/sa/settings/rate-limits/{id}")).await }
    pub async fn list_changes(&self, hours: u32, limit: u32) -> Result<Value> { admin_get(self.client, &format!("/sa/settings/rate-limits/changes?hours={hours}&limit={limit}")).await }
}

// ── WebhooksResource ─────────────────────────────────────────────────────────

pub struct WebhooksResource<'a> { client: &'a KuberNodes }
impl WebhooksResource<'_> {
    pub async fn list(&self) -> Result<Value> { admin_get(self.client, "/sa/settings/webhook-delivery").await }
    pub async fn get(&self, id: &str) -> Result<Value> { admin_get(self.client, &format!("/sa/settings/webhook-delivery/{id}")).await }
    pub async fn create(&self, data: Value) -> Result<Value> { admin_post(self.client, "/sa/settings/webhook-delivery", Some(data)).await }
    pub async fn update(&self, id: &str, data: Value) -> Result<Value> { admin_put(self.client, &format!("/sa/settings/webhook-delivery/{id}"), data).await }
    pub async fn delete(&self, id: &str) -> Result<Value> { admin_delete(self.client, &format!("/sa/settings/webhook-delivery/{id}")).await }
    pub async fn list_dlq(&self) -> Result<Value> { admin_get(self.client, "/sa/settings/webhook-delivery/dlq").await }
    pub async fn resolve_dlq(&self, id: &str) -> Result<Value> { admin_post(self.client, &format!("/sa/settings/webhook-delivery/dlq/{id}/resolve"), None).await }
    pub async fn list_changes(&self, hours: u32, limit: u32) -> Result<Value> { admin_get(self.client, &format!("/sa/settings/webhook-delivery/changes?hours={hours}&limit={limit}")).await }
}

// ── EmailTemplatesResource ───────────────────────────────────────────────────

pub struct EmailTemplatesResource<'a> { client: &'a KuberNodes }
impl EmailTemplatesResource<'_> {
    pub async fn list(&self) -> Result<Value> { admin_get(self.client, "/sa/settings/email-templates").await }
    pub async fn get(&self, id: &str) -> Result<Value> { admin_get(self.client, &format!("/sa/settings/email-templates/{id}")).await }
    pub async fn create(&self, data: Value) -> Result<Value> { admin_post(self.client, "/sa/settings/email-templates", Some(data)).await }
    pub async fn update(&self, id: &str, data: Value) -> Result<Value> { admin_put(self.client, &format!("/sa/settings/email-templates/{id}"), data).await }
    pub async fn delete(&self, id: &str) -> Result<Value> { admin_delete(self.client, &format!("/sa/settings/email-templates/{id}")).await }
    pub async fn preview(&self, id: &str, variables: Value) -> Result<Value> { admin_post(self.client, &format!("/sa/settings/email-templates/{id}/preview"), Some(serde_json::json!({"variables": variables}))).await }
    pub async fn list_changes(&self, hours: u32, limit: u32) -> Result<Value> { admin_get(self.client, &format!("/sa/settings/email-templates/changes?hours={hours}&limit={limit}")).await }
}

// ── IpRulesResource ──────────────────────────────────────────────────────────

pub struct IpRulesResource<'a> { client: &'a KuberNodes }
impl IpRulesResource<'_> {
    pub async fn list(&self) -> Result<Value> { admin_get(self.client, "/sa/settings/ip-rules").await }
    pub async fn get(&self, id: &str) -> Result<Value> { admin_get(self.client, &format!("/sa/settings/ip-rules/{id}")).await }
    pub async fn create(&self, data: Value) -> Result<Value> { admin_post(self.client, "/sa/settings/ip-rules", Some(data)).await }
    pub async fn update(&self, id: &str, data: Value) -> Result<Value> { admin_put(self.client, &format!("/sa/settings/ip-rules/{id}"), data).await }
    pub async fn delete(&self, id: &str) -> Result<Value> { admin_delete(self.client, &format!("/sa/settings/ip-rules/{id}")).await }
    pub async fn validate(&self, cidr: &str) -> Result<Value> { admin_post(self.client, "/sa/settings/ip-rules/validate", Some(serde_json::json!({"cidr": cidr}))).await }
    pub async fn list_changes(&self, hours: u32, limit: u32) -> Result<Value> { admin_get(self.client, &format!("/sa/settings/ip-rules/changes?hours={hours}&limit={limit}")).await }
}

// ── FeatureFlagsResource ─────────────────────────────────────────────────────

pub struct FeatureFlagsResource<'a> { client: &'a KuberNodes }
impl FeatureFlagsResource<'_> {
    pub async fn list(&self) -> Result<Value> { admin_get(self.client, "/flags").await }
    pub async fn get(&self, id: &str) -> Result<Value> { admin_get(self.client, &format!("/flags/{id}")).await }
    pub async fn create(&self, data: Value) -> Result<Value> { admin_post(self.client, "/flags", Some(data)).await }
    pub async fn update(&self, id: &str, data: Value) -> Result<Value> { admin_put(self.client, &format!("/flags/{id}"), data).await }
    pub async fn toggle(&self, id: &str) -> Result<Value> { admin_post(self.client, &format!("/flags/{id}/toggle"), None).await }
    pub async fn delete(&self, id: &str) -> Result<Value> { admin_delete(self.client, &format!("/flags/{id}")).await }
}

// ── AuditResource ────────────────────────────────────────────────────────────

pub struct AuditResource<'a> { client: &'a KuberNodes }
impl AuditResource<'_> {
    pub async fn query(&self, params: &[(&str, &str)]) -> Result<Value> {
        let qs: String = params.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("&");
        let path = if qs.is_empty() { "/sa/audit".to_owned() } else { format!("/sa/audit?{qs}") };
        admin_get(self.client, &path).await
    }
    pub async fn stats(&self) -> Result<Value> { admin_get(self.client, "/sa/audit/stats").await }
}

// ── HealthResource ───────────────────────────────────────────────────────────

pub struct HealthResource<'a> { client: &'a KuberNodes }
impl HealthResource<'_> {
    pub async fn status(&self) -> Result<Value> { admin_get(self.client, "/health").await }
    pub async fn redis(&self) -> Result<Value> { admin_get(self.client, "/health/redis").await }
    pub async fn phases(&self) -> Result<Value> { admin_get(self.client, "/health/phases").await }
}

// ── MaintenanceResource ──────────────────────────────────────────────────────

pub struct MaintenanceResource<'a> { client: &'a KuberNodes }
impl MaintenanceResource<'_> {
    pub async fn list(&self) -> Result<Value> { admin_get(self.client, "/sa/maintenance-windows").await }
    pub async fn create(&self, data: Value) -> Result<Value> { admin_post(self.client, "/sa/maintenance-windows", Some(data)).await }
    pub async fn delete(&self, id: &str) -> Result<Value> { admin_delete(self.client, &format!("/sa/maintenance-windows/{id}")).await }
}

// ── SsoResource ──────────────────────────────────────────────────────────────

pub struct SsoResource<'a> { client: &'a KuberNodes }
impl SsoResource<'_> {
    pub async fn list(&self) -> Result<Value> { admin_get(self.client, "/sa/sso/providers").await }
    pub async fn get(&self, id: &str) -> Result<Value> { admin_get(self.client, &format!("/sa/sso/providers/{id}")).await }
    pub async fn create(&self, data: Value) -> Result<Value> { admin_post(self.client, "/sa/sso/providers", Some(data)).await }
    pub async fn update(&self, id: &str, data: Value) -> Result<Value> { admin_put(self.client, &format!("/sa/sso/providers/{id}"), data).await }
    pub async fn delete(&self, id: &str) -> Result<Value> { admin_delete(self.client, &format!("/sa/sso/providers/{id}")).await }
    pub async fn set_active(&self, id: &str, active: bool) -> Result<Value> { admin_post(self.client, &format!("/sa/sso/providers/{id}/active"), Some(serde_json::json!({"active": active}))).await }
    pub async fn set_default(&self, id: &str) -> Result<Value> { admin_post(self.client, &format!("/sa/sso/providers/{id}/set-default"), None).await }
}

// ── BackupsResource ──────────────────────────────────────────────────────────

pub struct BackupsResource<'a> { client: &'a KuberNodes }
impl BackupsResource<'_> {
    pub async fn list(&self, db_id: &str) -> Result<Value> { admin_get(self.client, &format!("/databases/{db_id}/backups")).await }
    pub async fn get(&self, db_id: &str, backup_id: &str) -> Result<Value> { admin_get(self.client, &format!("/databases/{db_id}/backups/{backup_id}")).await }
    pub async fn trigger(&self, db_id: &str) -> Result<Value> { admin_post(self.client, &format!("/databases/{db_id}/backup"), None).await }
    pub async fn restore(&self, db_id: &str, backup_id: &str) -> Result<Value> { admin_post(self.client, &format!("/databases/{db_id}/restore"), Some(serde_json::json!({"backupId": backup_id}))).await }
}
