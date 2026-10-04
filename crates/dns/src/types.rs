/// A DNS zone as the provider lists it. `id` is the provider's own identifier — a
/// Cloudflare zone id, a Route53 hosted-zone id, a Hostinger domain — never a panel row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zone {
    pub id: String,
    pub name: String,
    pub status: Option<String>,
}

/// One DNS record value. Providers that group values into an rrset (Route53) or key rows
/// by name rather than id (OVH) still report one `Record` per value, so the panel can
/// point at a single row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub id: String,
    pub zone_id: String,
    pub record_type: String,
    pub name: String,
    pub content: String,
    pub ttl: Option<u32>,
    pub proxied: Option<bool>,
}

/// A create or replace request. `zone_id` is required by every provider; `proxied` is
/// honoured only where the provider has a proxy concept (Cloudflare).
#[derive(Debug, Clone, Default)]
pub struct RecordInput {
    pub zone_id: String,
    pub record_type: String,
    pub name: String,
    pub content: String,
    pub ttl: Option<u32>,
    pub proxied: Option<bool>,
}

/// Which existing row to change. Providers key rows differently — a Cloudflare record id,
/// a Route53 `type:name:value` triple, an OVH `subDomain`+`fieldType` pair — so the whole
/// identity travels with the request and each path uses the strongest hint it understands.
#[derive(Debug, Clone, Default)]
pub struct RecordTarget {
    pub zone_id: String,
    pub id: String,
    pub name: String,
    pub record_type: String,
    pub content: String,
}

impl RecordTarget {
    pub fn has_id(&self) -> bool {
        !self.id.trim().is_empty()
    }

    pub fn has_type(&self) -> bool {
        !self.record_type.trim().is_empty()
    }

    pub fn has_content(&self) -> bool {
        !self.content.trim().is_empty()
    }
}
