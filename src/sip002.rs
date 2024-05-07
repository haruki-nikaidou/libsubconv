use std::error::Error;
use std::fmt::{Display, Formatter};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SIP002URI {
    pub method: String,
    pub user_password: String,
    pub host: String,
    pub port: String,
    pub plugin: Option<String>,
    pub plugin_opts: Option<String>,
    pub node_name: String,
}

impl TryInto<String> for SIP002URI {
    type Error = ();

    fn try_into(self) -> Result<String, ()> {
        Ok(format!(
            "{}:{}@{}:{}?plugin={}&{}#{}",
            self.method,
            self.user_password,
            self.host,
            self.port,
            self.plugin.unwrap_or_default(),
            self.plugin_opts.unwrap_or_default(),
            self.node_name
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SIP002DesError {
    message: String,
}

impl Display for SIP002DesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SIP002DesError {}

impl TryFrom<&str> for SIP002URI {
    type Error = SIP002DesError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let url = Url::parse(value).map_err(|e| SIP002DesError {
            message: format!("parse url failed: {}", e),
        })?;
        let method = url.scheme().to_owned();
        let user_password = url.username().to_owned();
        let host = url.host_str().ok_or(SIP002DesError {
            message: "host is required".to_owned(),
        })?;
        let port = url.port().map(|p| p.to_string()).ok_or(SIP002DesError {
            message: "port is required".to_owned(),
        })?;
        let plugin = url.query_pairs()
            .find(|(k, _)| k == "plugin")
            .map(|(_, v)| v.to_string());
        let plugin_opts = url.query_pairs()
            .filter(|(k, _)| k != "plugin")
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");
        let node_name = url.fragment().unwrap_or_default().to_owned();
        Ok(SIP002URI {
            method,
            user_password,
            host: host.to_owned(),
            port,
            plugin,
            plugin_opts: Some(plugin_opts),
            node_name,
        })
    }
}