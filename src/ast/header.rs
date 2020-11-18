pub struct Header {}

impl Header {
    pub fn generate() -> String {
        format!(
            "{}",
            "#[allow(unused_imports)]\nuse std::collections::HashMap;"
        )
    }
}
