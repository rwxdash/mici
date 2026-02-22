use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
use std::collections::HashMap;

use crate::utils::traits::ExportAsHashMap;

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
}

impl Serialize for InitConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("InitConfiguration", 4)?;
        s.serialize_field("upstream_url", &self.upstream_url)?;
        s.serialize_field("upstream_cmd_path", &self.upstream_cmd_path)?;
        s.serialize_field("disable_cli_color", &self.disable_cli_color)?;
        s.serialize_field("disable_pager", &self.disable_pager)?;
        s.end()
    }
}
