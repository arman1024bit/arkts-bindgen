use crate::engine::item_container_gen::ItemContainerClient;
use crate::engine::type_result::TypeResult;
use cbindgen::ir::Type;
use std::collections::HashMap;

impl ItemContainerClient {
    /// 检测参数名是否符合数组模式（xxx_vec）
    /// 返回数组模式信息，包括基础名称和期望的长度参数名
    fn detect_array_pattern(&self, arg_name: &str) -> bool {
        if arg_name.is_empty() {
            return false;
        }
        arg_name.ends_with("vec") || arg_name.ends_with("_vec") || arg_name.ends_with("_array")
    }

    /// 检查下一个参数是否是当前数组参数的长度参数
    fn is_length_param(&self, next_arg_name: &str) -> bool {
        if next_arg_name.is_empty() {
            return false;
        }
        next_arg_name.ends_with("len")
            || next_arg_name.ends_with("_vec_len")
            || next_arg_name.ends_with("_vec_length")
    }

    /// 入参：
    /// pub type GetUserInfoCb = Option<
    ///     extern "C" fn(
    ///         context: *const c_void,
    ///         code: Error,
    ///         user_vec: *const UserInfo,
    ///         user_vec_len: size_t,
    ///
    ///         ret_info_vec: *const u8,
    ///         ret_info_vec_len: size_t,
    ///
    ///         extra: *const c_char,
    ///
    ///         failed_user_id_vec: *const *const c_char,
    ///         failed_user_id_vec_len: size_t,
    ///     ),
    /// >;
    /// 出参：
    /// 0:(context: void, code: Error, user_vec: Array<UserInfo>, ret_info_vec: Array<u8>, extra: string, failed_user_id_vec: Array<string>) => void;
    /// 1:方法返回值import信息 key:type_name value:import_content
    pub(crate) fn parser_aliased_type(
        &mut self,
        aliased: &Type,
    ) -> (String, HashMap<String, String>) {
        let mut content = String::from("(");
        let mut import_cache_map: HashMap<String, String> = HashMap::new();
        match aliased {
            Type::Ptr {
                ty,          //Box<Type>
                is_const,    //bool
                is_nullable, //bool
                is_ref,      //bool
            } => {}
            Type::Path(_) => {
                println!("");
            }
            Type::Primitive(_) => {
                println!("");
            }
            Type::Array(_, _) => {
                println!("");
            }
            Type::FuncPtr {
                ret,          //: Box<Type>,
                args,         //: Vec<(Option<String>, Type)>,
                is_nullable,  //: bool,
                never_return, //: bool,
            } => {
                let args_len = args.len();
                let mut skip_next = false;
                for index in 0..args_len {
                    if skip_next {
                        skip_next = false;
                        continue;
                    }
                    let args_item = &args[index];
                    let args_name = match &args_item.0 {
                        None => "",
                        Some(data) => data.as_str(),
                    };

                    let type_result: TypeResult = self.resolve_type(&args_item.1);

                    let array_pattern = self.detect_array_pattern(args_name);
                    if array_pattern {
                        // 检查下一个参数是否是长度参数
                        let next_arg_name =
                            args[index + 1].0.as_ref().map(|s| s.as_str()).unwrap_or("");

                        if self.is_length_param(next_arg_name) {
                            let array_type_name = format!("Array<{}>", type_result.ts_type_name());
                            content
                                .push_str(format!("{}: {}", args_name, array_type_name).as_str());
                            if index + 1 == args_len - 1 {
                                content.push_str(")");
                            } else {
                                content.push_str(", ");
                            }
                            // 跳过下一个长度参数
                            skip_next = true;
                        }
                    } else {
                        content.push_str(
                            format!("{}: {}", args_name, type_result.ts_type_name()).as_str(),
                        );
                        if index == args_len - 1 {
                            content.push_str(")");
                        } else {
                            content.push_str(", ");
                        }
                    }
                    import_cache_map.insert(
                        type_result.ts_type_name().to_string(),
                        type_result.ts_type_import_content().to_string(),
                    );
                }
                // for end
                let type_ret: TypeResult = self.resolve_type(ret.as_ref());
                content.push_str(format!(" => {};", type_ret.ts_type_name()).as_str());
                import_cache_map.insert(
                    type_ret.ts_type_name().to_string(),
                    type_ret.ts_type_import_content().to_string(),
                );
            }
        }
        (content, import_cache_map)
    }
}
