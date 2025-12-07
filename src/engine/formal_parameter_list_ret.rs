use std::collections::HashMap;

pub struct FormalParameterListResult {
    // 形参列表: (engine: Engine, user_id: string, clear_message: boolean, context: void, callback: voidCb)
    formal_param_list_content: String,
    // 入参列表：(engine, user_id, clear_message, context, callback)
    input_param_list_content: String,
    // 形参中对象的导出语句.key: type_name value:import content
    map: HashMap<String, String>,
}
impl FormalParameterListResult {
    pub fn new(
        formal_param_list_content: String,
        input_param_list_content: String,
        map: HashMap<String, String>,
    ) -> FormalParameterListResult {
        FormalParameterListResult {
            formal_param_list_content,
            input_param_list_content,
            map,
        }
    }

    pub fn formal_param_list_content(&self) -> &str {
        &self.formal_param_list_content
    }

    pub fn input_param_list_content(&self) -> &str {
        &self.input_param_list_content
    }

    pub fn map(&self) -> &HashMap<String, String> {
        &self.map
    }
}
