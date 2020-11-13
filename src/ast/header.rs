pub struct Header {}

impl Header {
    pub fn generate() -> String {
        format!(
            "{}\n{}",
            "#[allow(unused_imports)]\nuse spatial_macro::spatial_enum;",
            "#[allow(unused_imports)]\nuse std::collections::HashMap;"
        )
    }
}
