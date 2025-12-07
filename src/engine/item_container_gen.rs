use crate::engine::formal_parameter_list_ret::FormalParameterListResult;
use crate::engine::gen::field_utils;
use crate::engine::gen_field_info::GenFieldInfo;
use crate::engine::generator::{
    get_cpp_clz_name, get_documentation_content, get_field_content, get_final_name,
    CPP_HEADER_FILE_SUFFIX, CPP_SOURCE_FILE_SUFFIX, CPP_UTILS_CONTENT_TEMPLATE, NATIVE_HANDLE_NAME,
    TS_FILE_SUFFIX,
};
use crate::engine::listener::export_fn_name_listener;
use crate::engine::listener::export_fn_name_listener::{
    on_export_fn_name_listener, ExportFunctionNameListener,
};
use crate::engine::primitive_type::RPrimitiveType;
use crate::engine::type_result::TypeResult;
use crate::{
    cpp_clz_definition_template, cpp_clz_impl_template, napi_cpp_top_template,
    napi_header_file_template, napi_module_desc_template, napi_property_descriptor_item_template,
};
use cbindgen::ir::{
    AnnotationSet, Cfg, Documentation, Enum, EnumVariant, Field, Function, FunctionArgument,
    GenericArgument, GenericParams, ItemContainer, Literal, LiteralStructField, OpaqueItem,
    ReprAlign, Struct, Type, Typedef,
};
use cbindgen::Bindings;
use std::collections::HashMap;
use std::fmt::format;
use std::hash::Hash;
use std::path::Path;
use tokio::fs;

pub struct ItemContainerClient {
    ts_class_output_dir: String,
    ts_declaration_file: String,
    ts_export_class_name: String,
    //import testNapi from 'libentry.so';
    dynamic_library_export_statement: String,
    cpp_class_output_dir: String,
    // key : Struct, Enum, Union 名称
    global_import_cache_map: HashMap<String, String>,
    global_ignore_ts_functions: HashMap<String, ()>,
    module_name: String,
}

impl ItemContainerClient {
    pub fn set_export_fn_name_listener(&self, listener_opt: Option<ExportFunctionNameListener>) {
        export_fn_name_listener::set_export_fn_name_listener(listener_opt);
    }
}

impl ItemContainerClient {
    pub fn new(
        ts_class_output_dir: &str,
        ts_declaration_file: &str,
        cpp_class_output_dir: &str,
        ts_export_class_name: &str,
        dynamic_library_name: &str,
        module_name: &str,
        global_ignore_ts_functions: HashMap<String, ()>,
    ) -> Self {
        let dynamic_library_export = format!(
            "import {} from '{}';",
            NATIVE_HANDLE_NAME, dynamic_library_name
        );
        ItemContainerClient {
            ts_class_output_dir: ts_class_output_dir.to_string(),
            ts_declaration_file: ts_declaration_file.to_string(),
            ts_export_class_name: ts_export_class_name.to_string(),
            dynamic_library_export_statement: dynamic_library_export.to_string(),
            cpp_class_output_dir: cpp_class_output_dir.to_string(),
            global_import_cache_map: Default::default(),
            global_ignore_ts_functions,
            module_name: module_name.to_string(),
        }
    }

    pub fn check_insert_cache_map(&mut self, name: &str) -> String {
        if name.is_empty() {
            println!("check_insert_cache_map-->name is empty");
            return String::new();
        }
        if self.global_import_cache_map.contains_key(name) {
            let ret = self.global_import_cache_map.get(name);
            if let Some(data) = ret {
                if !data.is_empty() {
                    return data.clone();
                }
            }
        }
        let val = format!("import {{ {} }} from \"./{}\";\n", name, name);
        self.global_import_cache_map
            .insert(name.to_string(), val.clone());
        val
    }

    /// 常量、静态、结构体、联合、枚举、typedef 等
    pub fn parser_item_container(&self, bindings: &cbindgen::Bindings) {
        let items: &Vec<ItemContainer> = &bindings.items;
        for item_container in items {
            match item_container {
                ItemContainer::Constant(item) => {}
                ItemContainer::Static(item) => {}
                ItemContainer::OpaqueItem(item) => {}
                ItemContainer::Struct(item) => {
                    let _ = self.parser_struct(&item);
                }
                ItemContainer::Union(item) => {}
                ItemContainer::Enum(item) => {}
                ItemContainer::Typedef(item) => {}
            }
        }
    }

    // todo 每个结构体循环一次，文本的写入提出去
    pub async fn parser_struct(&self, item: &Struct) {
        let struct_name = get_final_name(item.export_name.as_str(), item.path.name.as_str());
        let ts_file_path = Path::new(self.ts_class_output_dir.as_str()).join(format!(
            "{}{}",
            struct_name,
            crate::engine::generator::TS_FILE_SUFFIX
        ));
        // 先收集所有字段的内容和 import 语句
        let mut all_imports = String::new();
        let mut all_field_content = String::new();

        for field_item in &item.fields {
            let info: GenFieldInfo = get_field_content(&field_item);
            all_field_content.push_str(info.ts_field_content.as_str());
            all_imports.push_str(info.ts_imports_content.as_str());
        }

        // 构建最终的 content，imports 在最顶部
        let mut content = String::new();

        // 在最顶部插入所有 import 语句
        content.push_str(all_imports.as_str());
        if !all_imports.is_empty() {
            content.push_str("\n");
        }

        // 然后是文档注释和类定义
        content.push_str(get_documentation_content(&item.documentation.doc_comment).as_str());
        content.push_str(
            String::from(format!(
                "{}{} extends Object {{\n",
                "export class ", struct_name
            ))
            .as_str(),
        );

        // 最后是所有字段
        content.push_str(all_field_content.as_str());
        content.push_str("}");

        fs::write(&ts_file_path, content.clone()).await.unwrap();
    }

    /// 生成类定义信息
    ///     继承关系 构造方法 析构函数 变量定义
    /// 类的实现
    ///     构造赋值 toValue 析构
    ///
    /// 循环一遍字段 需要拿到 【变量定义】【构造赋值】【toValue】【析构】
    pub fn parser_cpp_clz(&self, item: &Struct) -> String {
        let mut content = String::new();

        let struct_name = get_final_name(item.export_name.as_str(), item.path.name.as_str());
        let base_clz_name = struct_name.clone();
        let clz_name = get_cpp_clz_name(&struct_name);

        let mut member_variable_statement = String::new();
        let mut variable_assignment_statement = String::new();
        let mut set_property_statement = String::new();
        let mut destructor_statement = String::new();

        for field_item in &item.fields {
            let ret = field_utils::build_cpp_field_content(&field_item);
            member_variable_statement.push_str(ret.member_variable_statement.as_str());
            variable_assignment_statement.push_str(ret.variable_assignment_statement.as_str());
            set_property_statement.push_str(ret.set_property_statement.as_str());
            destructor_statement.push_str(ret.destructor_statement.as_str());
        }

        let clz_definition_content =
            cpp_clz_definition_template!(clz_name, base_clz_name, member_variable_statement);

        let clz_impl_content = cpp_clz_impl_template!(
            clz_name,
            base_clz_name,
            variable_assignment_statement,
            set_property_statement,
            destructor_statement
        );

        //
        content.push_str(clz_definition_content.as_str());
        content.push_str(clz_impl_content.as_str());
        content
    }

    pub async fn parser_enum(&self, item: &Enum) {
        let struct_name = get_final_name(item.export_name.as_str(), item.path.name.as_str());
        let ts_file_path = Path::new(self.ts_class_output_dir.as_str()).join(format!(
            "{}{}",
            struct_name,
            crate::engine::generator::TS_FILE_SUFFIX
        ));

        let mut all_field_content = String::new();

        for index in 0..item.variants.len() {
            let enum_variant = &item.variants[index];
            let field_name = get_final_name(
                enum_variant.export_name.as_str(),
                enum_variant.name.as_str(),
            );

            let mut ts_field_content = String::new();
            ts_field_content.push_str("\n");
            ts_field_content.push_str(
                get_documentation_content(&enum_variant.documentation.doc_comment).as_str(),
            );
            ts_field_content.push_str("  ");
            ts_field_content.push_str(field_name);
            ts_field_content.push_str(" = ");

            match &enum_variant.discriminant {
                None => {}
                Some(literal_item) => {
                    self.write_literal(literal_item, &mut ts_field_content);
                }
            }
            ts_field_content.push_str(",");
            ts_field_content.push_str("\n");
            all_field_content.push_str(ts_field_content.as_str());
        }

        // 构建最终的 content，imports 在最顶部
        let mut content = String::new();

        // 然后是文档注释和类定义
        content.push_str(get_documentation_content(&item.documentation.doc_comment).as_str());
        content.push_str(String::from(format!("{}{} {{\n", "export enum ", struct_name)).as_str());

        // 最后是所有字段
        content.push_str(all_field_content.as_str());
        content.push_str("}");

        fs::write(&ts_file_path, content.clone()).await.unwrap();
    }

    ///
    // OpaqueItem
    pub async fn parser_opaque_item(&self, item: &OpaqueItem) {
        let s = Struct::new(
            item.path.clone(),
            item.generic_params.clone(),
            vec![],
            false,
            false,
            None,
            false,
            item.cfg.clone(),
            item.annotations.clone(),
            item.documentation.clone(),
        );
        self.parser_struct(&s).await;
    }

    fn write_literal(&self, literal: &Literal, mut ts_field_content: &mut String) {
        match literal {
            Literal::Expr(v) => {
                ts_field_content.push_str(v);
            }
            Literal::Path {
                ref associated_to, //: Option<(Path, String)>,
                ref name,          //: String,
            } => {
                println!("");
            }
            Literal::PostfixUnaryOp {
                op,    //: &'static str,
                value, //: Box<Literal>,
            } => {
                ts_field_content.push_str(op);
                self.write_literal(value, ts_field_content);
                println!("");
            }
            Literal::BinOp {
                ref left,  //: Box<Literal>,
                op,        //: &'static str,
                ref right, //: Box<Literal>,
            } => {
                println!("");
                ts_field_content.push_str("(");
                self.write_literal(left, ts_field_content);
                ts_field_content.push_str(format!(" {}{}{} ", "{{", op, "}}").as_str());
                self.write_literal(right, ts_field_content);
                ts_field_content.push_str(")");
            }
            Literal::FieldAccess {
                ref base,  //: Box<Literal>,
                ref field, //: String,
            } => {
                ts_field_content.push_str("(");
                self.write_literal(base, ts_field_content);
                ts_field_content.push_str(")");
                println!("");
            }
            Literal::Struct {
                path,        //: Path,
                export_name, //: String,
                fields,      //: HashMap<String, LiteralStructField>,
            } => {
                println!("");
            }
            Literal::Cast {
                ty,    //: Type,
                value, //: Box<Literal>,
            } => {
                println!("");
            }
        }
    }

    /// 解析所有导出函数到 ts_export_class_name 类中
    /// 所有可导出 extern "C" 函数/方法的 IR；dynamic_symbols_names、语言 backend
    /// return property_descriptor_statement
    pub async fn parser_all_function(
        &mut self,
        b: &Bindings,
        declaration_import_dir: &str,
    ) -> String {
        if b.functions.is_empty() && b.globals.is_empty() {
            return "".to_string();
        }
        // key: type_name value:import content
        let mut all_fn_import_cache_map: HashMap<String, String> = HashMap::new(); //todo arman 重复insert

        let ts_export_file_path = Path::new(self.ts_class_output_dir.as_str()).join(format!(
            "{}{}",
            self.ts_export_class_name,
            crate::engine::generator::TS_FILE_SUFFIX
        ));

        // Declaration File
        let d_file_path = std::path::PathBuf::from(self.ts_declaration_file.clone());

        // 先收集所有方法的内容
        let mut all_fun_content = String::new();
        let mut all_d_fun_content = String::new();
        //
        let mut property_descriptor_statement = String::new();
        //
        for item in &b.functions {
            let raw_name = get_final_name("", item.path.name.as_str());
            if self.global_ignore_ts_functions.contains_key(raw_name) {
                continue;
            }

            let fun_name = on_export_fn_name_listener(raw_name);
            //
            property_descriptor_statement
                .push_str(napi_property_descriptor_item_template!(fun_name).as_str());
            //
            let mut ts_func_content = String::new();
            ts_func_content.push_str("\n");
            // if !function.annotations.should_export() {
            //     continue;
            // }
            ts_func_content
                .push_str(get_documentation_content(&item.documentation.doc_comment).as_str());
            ts_func_content.push_str(format!("  public {}", fun_name).as_str());

            // export const task1: (engine: Engine, a: number,
            // callback: ContactResponseCb) => ErrorCode;
            let mut d_file_fun_content = String::from(format!("export const {}: ", fun_name));
            //
            let ret = self.get_formal_parameter_list(&item.args);
            let fn_input_parameter_list_content = ret.input_param_list_content();
            let formal_param_list_content = ret.formal_param_list_content();
            for x in ret.map() {
                all_fn_import_cache_map.insert(
                    x.0.to_string(),
                    x.1.to_string(), //ts_type_import_content
                );
            }

            ts_func_content.push_str(format!("{}: ", formal_param_list_content).as_str());
            // :return val
            let return_val = self.resolve_type(&item.ret);
            ts_func_content.push_str(return_val.ts_type_name());
            all_fn_import_cache_map.insert(
                return_val.ts_type_name().to_string(),
                return_val.ts_type_import_content().to_string(),
            );

            // 组合ts倒出去类的方法和内部实现
            ts_func_content.push_str(" {\n");
            ts_func_content.push_str(
                format!(
                    "    return {}.{}{};",
                    NATIVE_HANDLE_NAME, fun_name, fn_input_parameter_list_content
                )
                .as_str(),
            );
            ts_func_content.push_str("\n");
            ts_func_content.push_str("  }\n");
            all_fun_content.push_str(ts_func_content.as_str());

            // 组合声明文件的内容
            d_file_fun_content.push_str(formal_param_list_content);
            d_file_fun_content.push_str(format!(" => {};", return_val.ts_type_name()).as_str());
            d_file_fun_content.push_str("\n");
            all_d_fun_content.push_str(d_file_fun_content.as_str());
        }

        {
            // 构建最终的 ts content，imports 在最顶部
            let mut content = String::from(self.dynamic_library_export_statement.as_str());
            content.push_str("\n");
            for import_kv in &all_fn_import_cache_map {
                content.push_str(import_kv.1.as_str());
            }

            content.push_str("\n");
            content.push_str(
                format!("{}{} {{\n", "export class ", self.ts_export_class_name).as_str(),
            );
            content.push_str("  public constructor() {}\n");
            content.push_str(all_fun_content.as_str());
            content.push_str("}");
            fs::write(&ts_export_file_path, content).await.unwrap();
        }
        {
            // 构建声明文件最终的 ts content，imports 在最顶部
            let mut content = String::new();
            for import_kv in &all_fn_import_cache_map {
                if import_kv.1.is_empty() {
                    continue;
                }
                //import { x } from "../../../ets/pages/x";
                content.push_str(
                    format!(
                        "import {{ {} }} from \"{}{}\";\n",
                        import_kv.0, declaration_import_dir, import_kv.0
                    )
                    .as_str(),
                );
            }

            content.push_str("\n");
            content.push_str(all_d_fun_content.as_str());
            fs::write(&d_file_path, content).await.unwrap();
        }
        property_descriptor_statement
    }

    /// 解析公共的Type，返回
    pub(crate) fn resolve_type(&mut self, ty: &Type) -> TypeResult {
        let mut ts_type_name: String = String::new();
        let mut ts_type_import_content: String = String::new();
        match ty {
            Type::Ptr {
                ty,          //Box<Type>
                is_const,    //bool
                is_nullable, //bool
                is_ref,      //bool
            } => {
                match ty.as_ref() {
                    Type::Ptr {
                        ty,          //Box<Type>
                        is_const,    //bool
                        is_nullable, //bool
                        is_ref,      //bool
                    } => {
                        let ret = self.resolve_type(ty);
                        ts_type_name = ret.ts_type_name().to_string();
                        ts_type_import_content = ret.ts_type_import_content().to_string();
                        // cpp_type_name = ret.cpp_type_name().to_string();
                        // cpp_type_import_content = ret.cpp_type_import_content().to_string();
                    }
                    Type::Array(b_type, constExpr) => {
                        println!("");
                    }
                    Type::Path(p) => {
                        // rust参数的Struct, Enum, Union,
                        ts_type_name = get_final_name(p.export_name(), p.path().name()).to_string();
                        ts_type_import_content = self.check_insert_cache_map(ts_type_name.as_str());
                    }
                    Type::Primitive(item) => {
                        let t = RPrimitiveType::from(item);
                        ts_type_name = t.get_ts_type_name().to_string();
                        // 基础类型不需要import
                        // ts_type_import_content = self.check_insert_cache_map(ts_type_name.as_str());
                    }
                    Type::FuncPtr { .. } => {
                        println!("");
                    }
                }
            }
            Type::Array(b_type, constExpr) => {
                println!("constExpr: {:?}", constExpr);
            }
            Type::Path(p) => {
                // rust参数的Struct, Enum, Union
                ts_type_name = get_final_name(p.export_name(), p.path().name()).to_string();
                ts_type_import_content = self.check_insert_cache_map(ts_type_name.as_str());
            }
            Type::Primitive(item) => {
                let t = RPrimitiveType::from(item);
                ts_type_name = t.get_ts_type_name().to_string();
                // 基础类型不需要import
                // ts_type_import_content = self.check_insert_cache_map(ts_type_name.as_str());
            }
            Type::FuncPtr {
                ret,          //: Box<Type>,
                args,         //: Vec<(Option<String>, Type)>,
                is_nullable,  //: bool,
                never_return, //: bool,
            } => {
                println!("ret: {:?}, args: {:?}", ret, args);
            }
        }
        TypeResult::new(ts_type_name, ts_type_import_content)
    }

    // todo 每个callback循环一次，文本的写入提出去
    pub async fn parser_typedef(&mut self, item: &Typedef) {
        let typedef_name = get_final_name(item.export_name.as_str(), item.path.name.as_str());

        let ts_typedef_file_path = Path::new(self.ts_class_output_dir.as_str()).join(format!(
            "{}{}",
            typedef_name,
            crate::engine::generator::TS_FILE_SUFFIX
        ));
        let mut ts_type_content = String::new();
        let aliased_type_pair: (String, HashMap<String, String>) =
            self.parser_aliased_type(&item.aliased);
        for import_kv in aliased_type_pair.1 {
            ts_type_content.push_str(import_kv.1.as_str());
        }
        ts_type_content.push_str("\n");
        ts_type_content
            .push_str(get_documentation_content(&item.documentation.doc_comment).as_str());
        ts_type_content.push_str(format!("{}{} = ", "export type ", typedef_name).as_str());
        ts_type_content.push_str(aliased_type_pair.0.as_str());
        ts_type_content.push_str("\n");
        fs::write(&ts_typedef_file_path, ts_type_content.clone())
            .await
            .unwrap();
    }

    /// 获取形参列表
    /// return : (engine: Engine, user_id: string, clear: boolean, context: void, callback: VoidCb)
    ///   // key: type_name value:import content
    ///   import_cache_map: HashMap<String, String>
    ///
    pub fn get_formal_parameter_list(
        &mut self,
        args: &Vec<FunctionArgument>,
    ) -> FormalParameterListResult {
        let mut formal_param_list_content = String::from("(");
        let mut input_param_list_content = String::from("(");
        // key: type_name value:import content
        let mut map: HashMap<String, String> = HashMap::new();
        //
        let args_len = args.len();
        for index in 0..args_len {
            let item = &args[index];
            // 形参名称
            let arg_name = item.name.clone().unwrap_or(String::from("XXXXX")); //todo bugtags --
                                                                               // 形参类型
            let type_ret: TypeResult = self.resolve_type(&item.ty);
            map.insert(
                type_ret.ts_type_name().to_string(),
                type_ret.ts_type_import_content().to_string(),
            );
            formal_param_list_content
                .push_str(format!("{}: {}", arg_name, type_ret.ts_type_name()).as_str());
            input_param_list_content.push_str(arg_name.as_str());
            if index == args_len - 1 {
                formal_param_list_content.push_str(")");
                input_param_list_content.push_str(")");
            } else {
                formal_param_list_content.push_str(", ");
                input_param_list_content.push_str(", ");
            }
        }
        FormalParameterListResult::new(formal_param_list_content, input_param_list_content, map)
    }

    ///
    /// module_name: entry  //.nm_modname = "entry",
    /// property_descriptor: { "add", nullptr, Add, nullptr, nullptr, nullptr, napi_default, nullptr },
    pub fn build_napi_module_register(
        &self,
        module_name: &str,
        property_descriptor: &str,
    ) -> String {
        napi_module_desc_template!(property_descriptor, module_name)
    }

    /// cpp_class_name          X 不需要后缀 ToDo 需要检查后缀
    /// include_guard:          TEST_H
    /// include_h_file          test.h
    /// namespace               test
    /// clz_impl_content        所有类 枚举等 所有方法 的CPP实现内容
    /// property_descriptor     { "add", nullptr, Add, nullptr, nullptr, nullptr, napi_default, nullptr },
    pub async fn build_napi_cpp(
        &self,
        cpp_class_name: &str,
        include_guard: &str,
        include_h_file: &str,
        namespace: &str,
        clz_fun_impl_content: &str,
        property_descriptor: &str,
    ) {
        let h_file = format!("{}{}", cpp_class_name, CPP_HEADER_FILE_SUFFIX);
        let header_file_path = Path::new(self.cpp_class_output_dir.as_str()).join(&h_file);
        if let Some(parent) = header_file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await.unwrap();
            }
        }
        let header_file_content =
            napi_header_file_template!(include_guard, include_h_file, namespace);
        fs::write(&header_file_path, header_file_content)
            .await
            .unwrap();

        //
        let source_file_path = Path::new(self.cpp_class_output_dir.as_str())
            .join(format!("{}{}", cpp_class_name, CPP_SOURCE_FILE_SUFFIX));
        if let Some(parent) = source_file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await.unwrap();
            }
        }

        let mut source_file_content = napi_cpp_top_template!(&h_file, namespace);
        // 工具类内容
        source_file_content.push_str(CPP_UTILS_CONTENT_TEMPLATE);
        source_file_content.push_str(clz_fun_impl_content);

        //  todo 类 napi转换代码

        // napi注册信息
        source_file_content.push_str(
            self.build_napi_module_register(self.module_name.as_str(), property_descriptor)
                .as_str(),
        );
        source_file_content.push_str("\n");
        source_file_content.push_str("}");
        fs::write(&source_file_path, source_file_content)
            .await
            .unwrap();
    }
}
