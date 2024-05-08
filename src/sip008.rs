use serde::{Deserialize, Serialize};
use base64::prelude::*;
use serde::de::Error;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SIP008Content {
    pub version: u8,
    pub servers: Vec<SIP008Node>,
    pub bytes_used: u128,
    pub bytes_remaining: u128,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SIP008Node {
    pub id: String,
    pub remarks: String,
    pub server: String,
    pub server_port: u16,
    pub password: String,
    pub method: String,
    pub plugin: Option<String>,
    pub plugin_opts: Option<String>,
}

impl TryInto<String> for SIP008Content {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        let json = serde_json::to_string(&self)?;
        let base64 = BASE64_STANDARD.encode(json);
        Ok(base64)
    }
}

impl TryFrom<&str> for SIP008Content {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let json = BASE64_STANDARD.decode(value.as_bytes())
            .map_err(|_| serde_json::Error::custom("Invalid base64"))?;
        serde_json::from_slice(&json)
    }
}