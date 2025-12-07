pub struct GenFieldInfo {
    ///    /**
    ///     * 消息id
    ///     */
    ///     messageId: string = "";\n
    pub ts_field_content: String,
    pub ts_imports_content: String,
    pub cpp_field_content: String,
}

impl GenFieldInfo {
    pub fn new() -> Self {
        GenFieldInfo {
            ts_field_content: "".to_string(),
            ts_imports_content: "".to_string(),
            cpp_field_content: "".to_string(),
        }
    }

    pub fn task(&self) {}
}
