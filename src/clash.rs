use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ClashSsNode {
    pub name: String,
    pub server: String,
    pub port: u16,
    pub password: String,
    pub cipher: String,
    pub udp: Option<bool>,
    pub plugin: Option<String>,
    #[serde(rename = "plugin-opts")]
    pub plugin_opts: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ClashSsrNode {
    pub name: String,
    pub server: String,
    pub port: u16,
    pub password: String,
    pub cipher: String,
    pub obfs: String,
    pub protocol: String,
    #[serde(rename = "obfs-param")]
    pub obfs_param: Option<String>,
    #[serde(rename = "protocol-param")]
    pub protocol_param: Option<String>,
    pub udp: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ClashTuicNode {
    pub name: String,
    pub server: String,
    pub port: u16,
    pub password: String,
    pub uuid: String,
    pub sni: Option<String>,
    #[serde(rename = "congestion-controller")]
    pub congestion_controller: Option<String>,
    pub reduce_rtt: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ClashVmessNode {
    pub name: String,
    pub server: String,
    pub port: u16,
    pub uuid: String,
    #[serde(rename = "alterId")]
    pub alter_id: u16,
    pub cipher: String,
    pub udp: Option<bool>,
    pub tls: Option<String>,
    #[serde(rename = "skip-cert-verify")]
    pub skip_cert_verify: Option<bool>,
    #[serde(rename = "servername")]
    pub server_name: Option<String>,
    pub network: Option<String>,
    #[serde(rename = "ws-opts")]
    pub ws_opts: Option<ClashVmessWsOptions>,
    #[serde(rename = "http-opts")]
    pub http_opts: Option<ClashVmessHttpOptions>,
    #[serde(rename = "h2-opts")]
    pub h2_opts: Option<String>,
    #[serde(rename = "quic-opts")]
    pub quic_opts: Option<String>,
    #[serde(rename = "grpc-opts")]
    pub grpc_opts: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ClashVmessWsOptions {
    pub path: String,
    pub headers: Option<HashMap<String, String>>,
    #[serde(rename = "max-early-data")]
    pub max_early_data: Option<u64>,
    #[serde(rename = "early-data-header-name")]
    pub early_data_header_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ClashVmessHttpOptions {
    pub method: String,
    pub path: Vec<String>,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ClashVmessH2Options {
    pub path: String,
    pub host: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum ClashNode {
    #[serde(rename = "ss")]
    Ss(ClashSsNode),
    #[serde(rename = "tuic")]
    Tuic(ClashTuicNode),
    #[serde(rename = "vmess")]
    Vmess(ClashVmessNode),
}