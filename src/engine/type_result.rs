#[derive(Default)]
pub struct TypeResult {
    ts_type_name: String,
    ts_type_import_content: String,
}

impl TypeResult {
    pub fn new(ts_type_name: String, ts_type_import_content: String) -> Self {
        TypeResult {
            ts_type_name,
            ts_type_import_content,
        }
    }

    pub fn ts_type_name(&self) -> &str {
        &self.ts_type_name
    }

    pub fn ts_type_import_content(&self) -> &str {
        &self.ts_type_import_content
    }
}
