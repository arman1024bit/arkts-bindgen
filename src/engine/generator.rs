use crate::engine::gen_field_info::GenFieldInfo;
use crate::engine::primitive_type::RPrimitiveType;
use cbindgen::ir::{
    AnnotationSet, Cfg, ConstExpr, Documentation, EnumVariant, Field, FunctionArgument,
    GenericParams, Item, ItemValue, Type, Typedef,
};
use std::collections::HashMap;

pub const TS_FILE_SUFFIX: &str = ".ts";
pub const DECLARATION_FILE_SUFFIX: &str = ".d.ts";
pub const CPP_SOURCE_FILE_SUFFIX: &str = ".cpp";
pub const CPP_HEADER_FILE_SUFFIX: &str = ".h";
pub const NATIVE_HANDLE_NAME: &str = "nativeNapi";
pub const CPP_THIS_PTR: &str = "this->";
pub const CHAR_PTR_TO_STRING_FUN: &str = "charToString";

pub const CPP_UTILS_CONTENT_TEMPLATE: &str = r#"// ----
class NativeBaseObject {
public:
    virtual ~NativeBaseObject() {}
    virtual napi_value toValue(napi_env env) = 0;
};

// --- std::string

std::string napiValueToString(napi_env env, napi_value value) {
    size_t length = 0;
    napi_get_value_string_utf8(env, value, nullptr, 0, &length);
    std::string ret_str(length, '\0');
    napi_get_value_string_utf8(env, value, (char *)ret_str.data(), length + 1, &length);
    return ret_str;
}

napi_value stringToNapiValue(napi_env env, std::string str) {
    if (str.empty()) {
        return {};
    }
    napi_value ret;
    napi_create_string_utf8(env, str.c_str(), str.length(), ret);
    return ret;
}

void napiSetPropertyString(napi_env env, napi_value obj, std::string k, std::string v) {
    if (k.empty()) {
        k = {};
    }
    if (v.empty()) {
        v = {};
    }
    napi_value key = stringToNapiValue(env, k);
    napi_value value = stringToNapiValue(env, v);
    napi_set_property(env, obj, key, value);
}

std::string napiGetPropertyString(napi_env env, napi_value obj, std::string k) {
    if (k.empty()) {
        return {};
    }
    napi_value key = stringToNapiValue(env, k);
    napi_value value;
    napi_get_property(env, obj, key, value);
    return napiValueToString(env, value);
}

napi_value napiGetPropertyNapiValue(napi_env env, napi_value obj, std::string k) {
    if (k.empty()) {
        return {};
    }
    napi_value key = stringToNapiValue(env, k);
    napi_value value;
    napi_get_property(env, obj, key, value);
    return value;
}

std::string charToString(const char *char_ptr) {
    if (char_ptr == nullptr) {
        return {};
    }
    try {
        return std::string(char_ptr);
    } catch (const std::exception &e) {
        return {};
    }
}

// --- int32_t

napi_value I32ToNapiValue(napi_env env, int32_t i) {
    napi_value ret;
    napi_create_int32(env, i, &ret);
    return ret;
}

int32_t napiValueToI32(napi_env env, napi_value v) {
    int32_t val;
    napi_get_value_int32(env, v, &val);
    return val;
}

void napiSetPropertyI32(napi_env env, napi_value obj, std::string k, int32_t v) {
    if (env == nullptr || obj == nullptr) {
        return;
    }
    if (k.empty()) {
        k = {};
    }
    napi_value key = stringToNapiValue(env, k);
    napi_value value = I32ToNapiValue(env, v);
    napi_set_property(env, obj, key, value);
}

int32_t napiGetPropertyI32(napi_env env, napi_value obj, std::string k) {
    if (k.empty()) {
        k = {};
    }
    napi_value key = stringToNapiValue(env, k);
    napi_value value;
    napi_get_property(env, obj, key, value);
    return napiValueToI32(env, value);
}

// --- double_t

double_t napiValueToDouble(napi_env env, napi_value value) {
    double_t ret;
    napi_get_value_double(env, value, &ret);
    return ret;
}

napi_value doubleToNapiValue(napi_env env, double_t num) {
    napi_value ret;
    napi_create_double(env, num, &ret);
    return ret;
}

double_t napiGetPropertyStringDouble(napi_env env, napi_value obj, std::string k) {
    if (k.empty()) {
        k = {};
    }
    napi_value key = stringToNapiValue(env, k);
    napi_value value;
    napi_get_property(env, obj, key, &value);
    return napiValueToDouble(env, value);
}

void napiSetPropertyDouble(napi_env env, napi_value obj, std::string k, double_t v) {
    if (k.empty()) {
        k = {};
    }
    napi_value key = stringToNapiValue(env, k);
    napi_value value = doubleToNapiValue(env, v);
    napi_set_property(env, obj, key, value);
}

// --- int64_t

int64_t napiValueToI64(napi_env env, napi_value value) {
    int64_t ret;
    napi_get_value_int64(env, value, &ret);
    return ret;
}

napi_value i64ToNapiValue(napi_env env, int64_t num) {
    napi_value ret;
    napi_create_int64(env, num, &ret);
    return ret;
}

void napiSetPropertyI64(napi_env env, napi_value obj, std::string k, int64_t v) {
    if (k.empty()) {
        k = {};
    }
    napi_value key = stringToNapiValue(env, k);
    napi_value value = i64ToNapiValue(env, v);
    napi_set_property(env, obj, key, value);
}

int64_t napiGetPropertyI64(napi_env env, napi_value obj, std::string k) {
    if (k.empty()) {
        k = {};
    }
    napi_value key = stringToNapiValue(env, k);
    napi_value value;
    napi_get_property(env, obj, key, &value);
    return napiValueToI64(env, value);
}

// --- bool

bool napiValueToBool(napi_env env, napi_value value) {
    bool ret;
    napi_get_value_bool(env, value, &ret);
    return ret;
}

napi_value boolToNapiValue(napi_env env, bool num) {
    napi_value ret;
    napi_get_boolean(env, num, &ret);
    return ret;
}

void napiSetPropertyBool(napi_env env, napi_value obj, std::string k, bool v) {
    if (k.empty()) {
        k = {};
    }
    napi_value key = stringToNapiValue(env, k);
    napi_value value = boolToNapiValue(env, v);
    napi_set_property(env, obj, key, value);
}

bool napiGetPropertyBool(napi_env env, napi_value obj, std::string k) {
    if (k.empty()) {
        k = {};
    }
    napi_value key = stringToNapiValue(env, k);
    napi_value value;
    napi_get_property(env, obj, key, &value);
    return napiValueToBool(env, value);
}

// --- char

void charPointPointFromValue(napi_env env, napi_value array_value, uint32_t array_len, char ***out_ptr) {
    if (array_len < 1) {
        return;
    }
    std::vector<std::string> vec;
    for (uint32_t i = 0; i < array_len; i++) {
        napi_value element;
        napi_get_element(env, array_value, i, &element);
        std::string str = napiValueToString(env, element);
        vec.push_back(str);
    }

    if (vec.size() > 0) {
        (*out_ptr) = new char *[vec.size()];
        for (uint32_t i = 0; i < vec.size(); i++) {
            size_t len = vec[i].length();
            (*out_ptr)[i] = new char[len + 1];
            std::strcpy((*out_ptr)[i], vec[i].c_str());
        }
    }
}

void freeCharPointPoint(char **char_ptr, size_t len) {
    if (char_ptr == nullptr || len < 1) {
        return;
    }
    for (size_t i = 0; i < len; i++) {
        delete[] char_ptr[i];
    }
    delete[] char_ptr;
}

void napiSetPropertyNativeObject(napi_env env, napi_value obj, std::string k, NativeBaseObject *nativeObj) {
    if (k.empty()) {
        k = {};
    }
    napi_value key = stringToNapiValue(env, k);
    if (nativeObj == nullptr) {
        napi_set_property(env, obj, key, nullptr);
    } else {
        napi_set_property(env, obj, key, nativeObj->toValue(env));
    }
}

bool isValidValue(napi_env env, napi_value obj) {
    if (env == nullptr || obj == nullptr) {
        return false;
    }
    napi_valuetype result;
    napi_typeof(env, obj, &result);
    if (napi_undefined == result || napi_null == result) {
        return false;
    }
    return true;
}
// ----

"#;

#[macro_export]
macro_rules! napi_module_desc_template {
    ($property_desc:expr, $module_name:expr) => {
        format!(
            r#"

EXTERN_C_START
static napi_value Init(napi_env env, napi_value exports) {{
    napi_property_descriptor desc[] = {{
        {}
    }};
    napi_define_properties(env, exports, sizeof(desc) / sizeof(desc[0]), desc);
    return exports;
}}
EXTERN_C_END

static napi_module demoModule = {{
    .nm_version = 1,
    .nm_flags = 0,
    .nm_filename = nullptr,
    .nm_register_func = Init,
    .nm_modname = "{}",
    .nm_priv = ((void*)0),
    .reserved = {{ 0 }},
}};

extern "C" __attribute__((constructor)) void RegisterEntryModule(void)
{{
    napi_module_register(&demoModule);
}}
        "#,
            $property_desc, $module_name
        )
    };
}

#[macro_export]
macro_rules! napi_header_file_template {
    ($include_guard:expr, $include_h_file:expr, $namespace:expr) => {
        format!(
            r#"
#ifndef {}
#define {}

#include <string>
#include <vector>
#include <stdint.h>
#include <cstdint>
#include "napi/native_api.h"
#include "{}"

namespace {} {{}}

#endif // namespace

            "#,
            $include_guard, $include_guard, $include_h_file, $namespace
        )
    };
}

#[macro_export]
macro_rules! napi_cpp_top_template {
    ($h_file:expr, $namespace:expr) => {
        format!(
            r#"
#include "{}"

namespace {} {{
            "#,
            $h_file, $namespace
        )
    };
}

#[macro_export]
macro_rules! napi_property_descriptor_item_template {
    ($fun_name:expr) => {
        format!(
            r#"
        {{ "{}", nullptr, {}, nullptr, nullptr, nullptr, napi_default, nullptr }},
            "#,
            $fun_name, $fun_name
        )
    };
}

#[macro_export]
macro_rules! cpp_clz_definition_template {
    ($clz_name:expr, $base_clz_name:expr,$member_variable_statement:expr) => {
        format!(
            r#"
class {} : public NativeBaseObject {{
public:
    {}(const struct {} *c);
    napi_value toValue(napi_env env) override;
    ~{}();

private:
    {}
}};
            "#,
            $clz_name, $clz_name, $base_clz_name, $clz_name, $member_variable_statement
        )
    };
}

#[macro_export]
macro_rules! cpp_clz_impl_template {
    ($clz_name:expr, $base_clz_name:expr, $variable_assignment_statement:expr, $set_property_statement:expr, $destructor_statement:expr) => {
        format!(
            r#"
{}::{}(const struct {} *c) {{
{}
}}

napi_value {}::toValue(napi_env env) {{
    napi_value ret;
    napi_create_object(env, &ret);
{}
    return ret;
}}

{}::~{}() {{
    {}
}}
            "#,
            $clz_name,
            $clz_name,
            $base_clz_name,
            $variable_assignment_statement,
            $clz_name,
            $set_property_statement,
            $clz_name,
            $clz_name,
            $destructor_statement
        )
    };
}

// this->info = new NativeInfo();
#[macro_export]
macro_rules! cpp_assignment_statement {
    ($field_name:expr, $clz_name:expr) => {
        format!(r#"    this->{} = new {}();"#, $field_name, $clz_name,)
    };
}

// this->type = c->type;
#[macro_export]
macro_rules! cpp_enum_statement {
    ($field_name:expr) => {
        format!(r#"    this->{} = c->{};"#, $field_name, $field_name,)
    };
}

#[macro_export]
macro_rules! set_property_template {
    ($method_name:expr, $field_name:expr) => {
        format!(
            r#"    {}(env, ret, "{}", this->{});"#,
            $method_name, $field_name, $field_name,
        )
    };
}

/// "    napiSetPropertyString(env, val, "id", this->id);"
#[macro_export]
macro_rules! set_property_string_template {
    ($field_name:expr) => {
        set_property_template!("napiSetPropertyString", $field_name)
    };
}

///"    napisetPropertyInt32(env, val, "checkTotalCount", this->check_total_count);"
#[macro_export]
macro_rules! set_property_i32_template {
    ($field_name:expr) => {
        set_property_template!("napiSetPropertyI32", $field_name)
    };
}

#[macro_export]
macro_rules! set_property_i64_template {
    ($field_name:expr) => {
        set_property_template!("napiSetPropertyI64", $field_name)
    };
}

/// napiSetPropertyNativeObject(env, ret, "collection_type", this->collection_type);
#[macro_export]
macro_rules! set_property_obj_template {
    ($field_name:expr) => {
        set_property_template!("napiSetPropertyNativeObject", $field_name)
    };
}

/// napiSetPropertyBool(env, ret, "expired", this->expired);
#[macro_export]
macro_rules! set_property_bool_template {
    ($field_name:expr) => {
        set_property_template!("napiSetPropertyBool", $field_name)
    };
}

#[macro_export]
macro_rules! set_property_double_template {
    ($field_name:expr) => {
        set_property_template!("napiSetPropertyDouble", $field_name)
    };
}

#[macro_export]
macro_rules! destructor_statement_template {
    ($field_name:expr) => {
        format!(
            r#"if (this->{} != nullptr) {{
        delete this->{};
    }}"#,
            $field_name, $field_name,
        )
    };
}

pub fn get_final_name<'a>(export_name: &'a str, path_name: &'a str) -> &'a str {
    if export_name.is_empty() {
        path_name
    } else {
        export_name
    }
}

pub fn get_cpp_clz_name(struct_name: &str) -> String {
    format!("Native{}", struct_name)
}

/// doc_comment : Struct or Filed
pub fn get_documentation_content(doc_comment: &Vec<String>) -> String {
    let mut doc_content = String::new();
    doc_content.push_str("  ");
    doc_content.push_str("/**");
    doc_content.push_str("\n");
    let mut doc_not_empty = false;
    for doc in doc_comment {
        doc_content.push_str("   * ");
        if !doc.is_empty() {
            doc_not_empty = true;
        }
        doc_content.push_str(doc);
        doc_content.push_str("\n");
    }
    doc_content.push_str("   */");
    doc_content.push_str("\n");
    if doc_not_empty {
        doc_content
    } else {
        String::new()
    }
}
/// cpp 类也制定目录
/// 注释 和 field 注释需要存在缓存中
pub fn get_field_content(field: &Field) -> GenFieldInfo {
    let field_name = field.name.as_str();
    let mut ts_imports_content = String::new();
    // key : Struct, Enum, Union 名称
    let mut import_cache_map: HashMap<String, ()> = HashMap::new();

    let mut ts_field_content = String::new();
    ts_field_content.push_str("\n");
    ts_field_content.push_str(get_documentation_content(&field.documentation.doc_comment).as_str());
    ts_field_content.push_str("  ");
    ts_field_content.push_str(field_name);
    ts_field_content.push_str(": ");

    let mut cpp_field_content = String::new();
    cpp_field_content.push_str("\n");
    cpp_field_content
        .push_str(get_documentation_content(&field.documentation.doc_comment).as_str());
    cpp_field_content.push_str("    ");

    // println!("field.cfg: {:?}", field.cfg);// 继续全部为none

    //  { annotations: {}, must_use: false, deprecated: None }
    // println!("field.annotations: {:?}", field.annotations);

    let mut cpp_field_content = String::new();
    let mut cpp_construct_content = String::new();
    match &field.ty {
        Type::Ptr {
            ty,          //Box<Type>
            is_const,    //bool
            is_nullable, //bool
            is_ref,      //bool
        } => {
            // 处理指针内部的类型
            match ty.as_ref() {
                Type::Primitive(primitive_type) => {
                    // 例如: const char* -> string
                    let t = RPrimitiveType::from(primitive_type);
                    ts_field_content.push_str(t.get_ts_type_name());
                    ts_field_content.push_str(" = ");
                    ts_field_content.push_str(t.get_ts_type_default_name());
                    ts_field_content.push_str(";");

                    cpp_field_content
                        .push_str(format!("{} {};", t.get_cpp_type_name(), field_name).as_str());
                    cpp_construct_content
                        .push_str(format!("    {}{} = ", CPP_THIS_PTR, field_name).as_str());
                }
                Type::Path(p) => {
                    //GenericPath
                    // rust参数的Struct, Enum, Union,
                    let name = get_final_name(p.export_name(), p.path().name());
                    if import_cache_map.contains_key(name) {
                        // 存在，不生成 import
                    } else {
                        import_cache_map.insert(name.to_string(), ());
                        ts_imports_content.push_str(
                            format!("import {{ {} }} from \"./{}\";\n", name, name).as_str(),
                        );
                    }
                    ts_field_content.push_str(name);
                    ts_field_content.push_str(";");

                    cpp_field_content.push_str(format!("{} {};", name, field_name).as_str());
                }
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
                            println!("");
                        }
                        Type::Path(p) => {
                            // rust参数的Struct, Enum, Union,
                            let name = get_final_name(p.export_name(), p.path().name());
                            if import_cache_map.contains_key(name) {
                                // 存在，不生成 import
                            } else {
                                import_cache_map.insert(name.to_string(), ());
                                ts_imports_content.push_str(
                                    format!("import {{ {} }} from \"./{}\";\n", name, name)
                                        .as_str(),
                                );
                            }
                            ts_field_content.push_str(name);
                            ts_field_content.push_str(";");

                            cpp_field_content
                                .push_str(format!("{} {};", name, field_name).as_str());
                        }
                        Type::Primitive(primitive_type) => {
                            let t = RPrimitiveType::from(primitive_type);
                            ts_field_content.push_str(t.get_ts_type_name());
                            ts_field_content.push_str(" = ");
                            ts_field_content.push_str(t.get_ts_type_default_name());
                            ts_field_content.push_str(";");

                            cpp_field_content.push_str(
                                format!("{} {};", t.get_cpp_type_name(), field_name).as_str(),
                            );
                        }
                        Type::Array(b_type, constExpr) => {
                            println!("");
                        }
                        Type::FuncPtr {
                            ret,          //: Box<Type>
                            args,         //: Vec<(Option<String>, Type)>
                            is_nullable,  //: bool
                            never_return, //: bool
                        } => {
                            println!("");
                        }
                    }
                }
                Type::Array(b_type, constExpr) => {
                    println!("");
                }
                Type::FuncPtr {
                    ret,          //: Box<Type>
                    args,         //: Vec<(Option<String>, Type)>
                    is_nullable,  //: bool
                    never_return, //: bool
                } => {
                    println!("");
                }
            }
        }
        Type::Path(p) => {
            // rust参数的Struct, Enum, Union,
            let name = get_final_name(p.export_name(), p.path().name());
            if import_cache_map.contains_key(name) {
                // 存在，不生成 import
            } else {
                import_cache_map.insert(name.to_string(), ());
                ts_imports_content
                    .push_str(format!("import {{ {} }} from \"./{}\";\n", name, name).as_str());
            }
            ts_field_content.push_str(name);
            ts_field_content.push_str(";");

            cpp_field_content.push_str(format!("{} {};", name, field_name).as_str());
        }
        Type::Primitive(primitive_type) => {
            let t = RPrimitiveType::from(primitive_type);
            ts_field_content.push_str(t.get_ts_type_name());
            ts_field_content.push_str(" = ");
            ts_field_content.push_str(t.get_ts_type_default_name());
            ts_field_content.push_str(";");

            cpp_field_content
                .push_str(format!("{} {};", t.get_cpp_type_name(), field_name).as_str());
        }
        Type::Array(b_type, constExpr) => {
            match constExpr {
                ConstExpr::Name(name) => {}
                ConstExpr::Value(value) => {}
            }
            ts_field_content.push_str("Array<any> = [];");
            cpp_field_content.push_str(format!("NativeList *{};", field_name).as_str());
        }
        Type::FuncPtr {
            ret,          //: Box<Type>,
            args,         //: Vec<(Option<String>, Type)>,
            is_nullable,  //: bool,
            never_return, //: bool,
        } => {
            ts_field_content.push_str("Function = null;");
            cpp_field_content.push_str("Function");
        }
    }
    ts_field_content.push_str("\n");
    GenFieldInfo {
        ts_field_content,
        ts_imports_content,
        cpp_field_content,
    }
}

pub fn get_enum_field_content(field: &EnumVariant) {
    let field_name = field.name.as_str();
    let mut ts_field_content = String::new();
    ts_field_content.push_str("\n");
    ts_field_content.push_str(get_documentation_content(&field.documentation.doc_comment).as_str());
    ts_field_content.push_str("  ");
    ts_field_content.push_str(field_name);
    ts_field_content.push_str(" = ");
}
