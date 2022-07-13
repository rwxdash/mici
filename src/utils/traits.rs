use std::collections::HashMap;

pub trait ExportAsHashMap {
    fn as_hash_map(&self) -> HashMap<&str, &str>;
}
