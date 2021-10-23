use std::collections::HashMap;

pub struct BaseCommand {
    pub name: &'static str,
    pub description: &'static str,
    pub synopsis: &'static str,
    pub options: &'static str,
    pub usage: &'static str,
}

pub trait ExportAsHashMap {
    fn as_hash_map(&self) -> HashMap<&str, &str>;
}

impl ExportAsHashMap for BaseCommand {
    fn as_hash_map(&self) -> HashMap<&str, &str> {
        let mut content: HashMap<&str, &str> = HashMap::new();

        content.insert("name", self.name.trim());
        content.insert("description", self.description.trim());
        content.insert("synopsis", self.synopsis.trim());
        content.insert("options", self.options.trim());
        content.insert("usage", self.usage.trim());

        return content;
    }
}
