use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
use std::collections::HashMap;
use std::fmt;

use crate::utils::traits::ExportAsHashMap;

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogTimer {
    #[default]
    Wallclock,
    Uptime,
    None,
}

impl fmt::Display for LogTimer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogTimer::Wallclock => write!(f, "wallclock"),
            LogTimer::Uptime => write!(f, "uptime"),
            LogTimer::None => write!(f, "none"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    #[default]
    Info,
    Debug,
    Warn,
    Error,
    Trace,
    Off,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Info => write!(f, "info"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
            LogLevel::Trace => write!(f, "trace"),
            LogLevel::Off => write!(f, "off"),
        }
    }
}

pub struct BaseCommand {
    pub name: &'static str,
    pub description: &'static str,
    pub synopsis: &'static str,
    pub options: &'static str,
    pub usage: &'static str,
}

impl ExportAsHashMap for BaseCommand {
    fn as_hash_map(&self) -> HashMap<&str, &str> {
        let mut content: HashMap<&str, &str> = HashMap::new();

        content.insert("name", self.name.trim());
        content.insert("description", self.description.trim());
        content.insert("synopsis", self.synopsis.trim());
        content.insert("options", self.options.trim());
        content.insert("usage", self.usage.trim());

        content
    }
}

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct InitConfiguration {
    pub upstream_url: Option<String>,
    pub upstream_cmd_path: Option<String>,
    pub disable_cli_color: Option<bool>,
    pub disable_pager: Option<bool>,
    pub log_timer: Option<LogTimer>,
    pub log_level: Option<LogLevel>,
}

impl Serialize for InitConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("InitConfiguration", 6)?;
        s.serialize_field("upstream_url", &self.upstream_url)?;
        s.serialize_field("upstream_cmd_path", &self.upstream_cmd_path)?;
        s.serialize_field("disable_cli_color", &self.disable_cli_color)?;
        s.serialize_field("disable_pager", &self.disable_pager)?;
        s.serialize_field("log_timer", &self.log_timer)?;
        s.serialize_field("log_level", &self.log_level)?;
        s.end()
    }
}
