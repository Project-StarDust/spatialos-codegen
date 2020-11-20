pub struct Header {}

impl Header {
    pub fn generate() -> String {
        "#[allow(unused_imports)]\nuse std::collections::HashMap;".to_string()
    }
}
