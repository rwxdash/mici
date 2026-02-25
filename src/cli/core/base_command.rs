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

impl InitConfiguration {
    pub fn format_config_yaml(&self) -> String {
        let format_optional = |val: &Option<String>| match val {
            Some(v) => format!("\"{}\"", v),
            None => "null".to_string(),
        };

        let format_bool = |val: &Option<bool>| match val {
            Some(true) => "true".to_string(),
            Some(false) => "false".to_string(),
            None => "false".to_string(),
        };

        format!(
            r#"##  ==================================================
##  mici Configuration
##  Global settings for all mici commands
##  ==================================================

##  Upstream Repository
#
#   upstream_url: String
#         [Optional]  default: null
#         Git repository URL where your commands are stored
#         Used by `mici fetch` to pull commands from a remote
#   upstream_cmd_path: String
#         [Optional]  default: null
#         Path to the commands directory within the repository
#
upstream_url: {upstream_url}
upstream_cmd_path: {upstream_cmd_path}

##  Terminal Settings
#
#   disable_cli_color: bool
#         [Optional]  default: false
#         Disable colored output in the terminal
#   disable_pager: bool
#         [Optional]  default: false
#         Disable the pager for long output (e.g., help text)
#
disable_cli_color: {disable_cli_color}
disable_pager: {disable_pager}

##  Logging
#
#   log_timer: String
#         [Optional]  default: "wallclock"
#         Timer style for tracing log output
#         Options:
#           "wallclock" - Full timestamps (e.g., 2026-02-23T14:30:00Z)
#           "uptime"    - Time since process start (e.g., 0.003s)
#           "none"      - No timestamps in log output
#   log_level: String
#         [Optional]  default: "info"
#         Minimum log level for tracing output
#         Options:
#           "trace" - Most verbose, includes all messages
#           "debug" - Detailed diagnostic messages
#           "info"  - General informational messages
#           "warn"  - Warning messages only
#           "error" - Error messages only
#           "off"   - Suppress all log output (silent mode)
#
log_timer: {log_timer}
log_level: {log_level}
"#,
            upstream_url = format_optional(&self.upstream_url),
            upstream_cmd_path = format_optional(&self.upstream_cmd_path),
            disable_cli_color = format_bool(&self.disable_cli_color),
            disable_pager = format_bool(&self.disable_pager),
            log_timer = self
                .log_timer
                .as_ref()
                .map_or("wallclock".to_string(), |t| t.to_string()),
            log_level = self
                .log_level
                .as_ref()
                .map_or("info".to_string(), |l| l.to_string()),
        )
    }
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
