use serde::{Deserialize, Serialize};
use base64::prelude::*;
use serde::de::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct VMessLinkContent {
    /// always `"2"`
    pub v: String,

    /// the name of the node
    pub ps: String,

    /// IP or domain of the endpoint
    pub add: String,

    /// port of the endpoint
    pub port: String,

    /// UUID
    pub id: String,

    /// alterId
    pub aid: String,

    #[serde(default = "default_scy")]
    /// security type, default is "none"
    pub scy: String,

    /// network type
    pub net: String,

    #[serde(rename = "type", default = "default_type_")]
    /// Type of camouflage, default is "none"
    pub type_: String,

    /// The domain of the camouflaged website
    /// 1. http(tcp): Host
    /// 2. ws: Host
    /// 3. h2: Host
    /// 4. QUIC: security
    pub host: String,

    /// Path of the camouflaged website
    /// 1. ws: Path
    /// 2. h2: Path
    /// 3. QUIC: key
    /// 4. KCP: seed
    /// 5. grpc: serviceName
    pub path: String,

    pub tls: String,

    pub sni: Option<String>,

    pub alpn: Option<String>,

    /// fingerprint
    pub fp: Option<String>,
}

pub fn default_scy() -> String { "none".to_owned() }

pub fn default_type_() -> String { "none".to_owned() }

impl TryInto<String> for VMessLinkContent {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        let json = serde_json::to_string(&self)?;
        let base64 = BASE64_STANDARD.encode(json.as_bytes());
        Ok(format!(
            "vmess://{}",
            base64
        ))
    }
}

impl TryFrom<&str> for VMessLinkContent {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // check if the value is a vmess link
        if !value.starts_with("vmess://") {
            return Err(serde_json::Error::custom("A VMess link must start with `vmess://`"));
        }
        let base64 = value.trim_start_matches("vmess://");
        let json = BASE64_STANDARD.decode(base64.as_bytes())
            .map_err(|_| serde_json::Error::custom("Invalid base64"))?;
        serde_json::from_slice(&json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_content() -> VMessLinkContent {
        VMessLinkContent {
            v: "2".to_owned(),
            ps: "test".to_owned(),
            add: "11.4.5.14".to_owned(),
            port: "19198".to_owned(),
            id: "0398607c-e7d4-493d-8cf2-23ff81b4fce6".to_owned(),
            aid: "0".to_owned(),
            scy: "none".to_owned(),
            net: "tcp".to_owned(),
            type_: "none".to_owned(),
            host: "".to_string(),
            path: "".to_string(),
            tls: "".to_string(),
            sni: Some("".to_string()),
            alpn: Some("".to_string()),
            fp: Some("".to_string()),
        }
    }

    #[test]
    fn test_serialize() {
        let content = get_content();
        let s: String = content.try_into().unwrap();
        let expected = "vmess://eyJ2IjoiMiIsInBzIjoidGVzdCIsImFkZCI6IjExLjQuNS4xNCIsInBvcnQiOiIxOTE5OCIsImlkIjoiMDM5ODYwN2MtZTdkNC00OTNkLThjZjItMjNmZjgxYjRmY2U2IiwiYWlkIjoiMCIsInNjeSI6Im5vbmUiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIiwic25pIjoiIiwiYWxwbiI6IiIsImZwIjoiIn0=";
        assert_eq!(s, expected);
    }

    #[test]
    fn test_deserialize() {
        let from = "vmess://eyJhZGQiOiIxMS40LjUuMTQiLCJhaWQiOiIwIiwiYWxwbiI6IiIsImZwIjoiIiwiaG9zdCI6IiIsImlkIjoiMDM5ODYwN2MtZTdkNC00OTNkLThjZjItMjNmZjgxYjRmY2U2IiwibmV0IjoidGNwIiwicGF0aCI6IiIsInBvcnQiOiIxOTE5OCIsInBzIjoidGVzdCIsInNjeSI6Im5vbmUiLCJzbmkiOiIiLCJ0bHMiOiIiLCJ0eXBlIjoibm9uZSIsInYiOiIyIn0=";
        let expected = get_content();
        let content: VMessLinkContent = from.try_into().unwrap();
        assert_eq!(content, expected);
    }
}