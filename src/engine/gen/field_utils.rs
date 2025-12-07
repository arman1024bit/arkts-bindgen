use crate::engine::generator::{
    get_cpp_clz_name, get_documentation_content, get_final_name, CHAR_PTR_TO_STRING_FUN,
    CPP_THIS_PTR,
};
use crate::engine::primitive_type::RPrimitiveType;
use crate::{
    cpp_assignment_statement, cpp_enum_statement, set_property_bool_template,
    set_property_i32_template, set_property_obj_template,
};
use crate::{
    destructor_statement_template, set_property_double_template, set_property_string_template,
    set_property_template,
};
use cbindgen::declarationtyperesolver::DeclarationType;
use cbindgen::ir::{ConstExpr, Field, GenericPath, Type};

/// return
///     std::string user_id;
///     GroupType group_type;
///     NativeBaseList *group_members;
fn get_cpp_parameter_declaration(field: &Field) -> String {
    let field_name = field.name.as_str();

    let mut content = String::new();
    content.push_str(get_documentation_content(&field.documentation.doc_comment).as_str());
    content.push_str("    ");

    // println!("field.cfg: {:?}", field.cfg);// 继续全部为none

    //  { annotations: {}, must_use: false, deprecated: None }
    // println!("field.annotations: {:?}", field.annotations);

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
                    content.push_str(format!("{} {};", t.get_cpp_type_name(), field_name).as_str());
                }
                Type::Path(p) => {
                    //GenericPath
                    // rust参数的Struct, Enum, Union,
                    let name = get_final_name(p.export_name(), p.path().name());
                    content.push_str(format!("{} {};", name, field_name).as_str());
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
                            content.push_str(format!("{} {};", name, field_name).as_str());
                        }
                        Type::Primitive(primitive_type) => {
                            let t = RPrimitiveType::from(primitive_type);
                            content.push_str(
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
            content.push_str(format!("{} {};", name, field_name).as_str());
        }
        Type::Primitive(primitive_type) => {
            let t = RPrimitiveType::from(primitive_type);
            content.push_str(format!("{} {};", t.get_cpp_type_name(), field_name).as_str());
        }
        Type::Array(b_type, constExpr) => {
            match constExpr {
                ConstExpr::Name(name) => {}
                ConstExpr::Value(value) => {}
            }
            content.push_str(format!("NativeList *{};", field_name).as_str());
        }
        Type::FuncPtr {
            ret,          //: Box<Type>,
            args,         //: Vec<(Option<String>, Type)>,
            is_nullable,  //: bool,
            never_return, //: bool,
        } => {
            content.push_str("Function");
        }
    }
    content.push_str("\n");

    content
}

/// return this->user_id = charToStringFromSafety(c->user_id);
pub fn get_cpp_member_variable_statement(field: &Field) -> String {
    let field_name = field.name.as_str();

    let mut content = String::new();
    // println!("field.cfg: {:?}", field.cfg);// 继续全部为none
    //  { annotations: {}, must_use: false, deprecated: None }
    // println!("field.annotations: {:?}", field.annotations);

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
                    content
                        .push_str(get_cpp_basic_type_assignment_statement(field_name, &t).as_str());
                }
                Type::Path(p) => {
                    content.push_str(
                        get_cpp_clz_enum_type_assignment_statement(field_name, &p).as_str(),
                    );
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
                            content.push_str(
                                get_cpp_clz_enum_type_assignment_statement(field_name, &p).as_str(),
                            );
                        }
                        Type::Primitive(primitive_type) => {
                            let t = RPrimitiveType::from(primitive_type);
                            content.push_str(
                                get_cpp_basic_type_assignment_statement(field_name, &t).as_str(),
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
            content.push_str(get_cpp_clz_enum_type_assignment_statement(field_name, &p).as_str());
        }
        Type::Primitive(primitive_type) => {
            let t = RPrimitiveType::from(primitive_type);
            content.push_str(get_cpp_basic_type_assignment_statement(field_name, &t).as_str());
        }
        Type::Array(b_type, constExpr) => {
            match constExpr {
                ConstExpr::Name(name) => {}
                ConstExpr::Value(value) => {}
            }
            content.push_str(format!("NativeList *{};", field_name).as_str());
        }
        Type::FuncPtr {
            ret,          //: Box<Type>,
            args,         //: Vec<(Option<String>, Type)>,
            is_nullable,  //: bool,
            never_return, //: bool,
        } => {
            content.push_str("Function");
        }
    }
    content.push_str("\n");

    content
}

//todo 注释需要修改
/// return "    this->user_id = charToStringFromSafety(c->user_id);"
fn get_cpp_basic_type_assignment_statement(field_name: &str, t: &RPrimitiveType) -> String {
    let mut content = String::new();
    //"     this->id = "
    content.push_str(format!("    {}{} = ", CPP_THIS_PTR, field_name).as_str());
    match t {
        RPrimitiveType::Integer {
            zeroable: _,
            signed: _,
            kind: _,
        }
        | RPrimitiveType::Double
        | RPrimitiveType::Float
        | RPrimitiveType::Bool
        | RPrimitiveType::Char32
        | RPrimitiveType::Void => {
            content.push_str(format!("c->{};", field_name).as_str());
        }
        RPrimitiveType::UChar | RPrimitiveType::SChar | RPrimitiveType::Char => {
            content.push_str(format!("{}(c->{});", CHAR_PTR_TO_STRING_FUN, field_name).as_str());
        }
        RPrimitiveType::VaList => {
            content.push_str(format!("c->VaList{};", field_name).as_str());
        }
        RPrimitiveType::PtrDiffT => {
            content.push_str(format!("c->PtrDiffT{};", field_name).as_str());
        }
    }
    content
}

/// return
/// "    napisetPropertyInt32(env, val, "checkTotalCount", this->check_total_count);"
/// "    napiSetPropertyString(env, val, "id", this->id);"
/// "    napisetPropertyBool(env, val, "isRead", this->is_read);"
fn get_cpp_set_property_statement(field_name: &str, t: &RPrimitiveType) -> String {
    let mut content = String::new();
    match t {
        RPrimitiveType::UChar | RPrimitiveType::SChar | RPrimitiveType::Char => {
            content.push_str(set_property_string_template!(field_name).as_str());
        }
        RPrimitiveType::VaList => {
            content.push_str(format!("c->VaList{};", field_name).as_str());
        }
        RPrimitiveType::PtrDiffT => {
            content.push_str(format!("c->PtrDiffT{};", field_name).as_str());
        }
        RPrimitiveType::Void => {}
        RPrimitiveType::Bool => {
            content.push_str(set_property_bool_template!(field_name).as_str());
        }
        RPrimitiveType::Char32 => {
            content.push_str(set_property_i32_template!(field_name).as_str());
        }
        RPrimitiveType::Float => {
            content.push_str(set_property_double_template!(field_name).as_str());
        }
        RPrimitiveType::Double => {
            content.push_str(set_property_double_template!(field_name).as_str());
        }
        RPrimitiveType::Integer { .. } => {
            content.push_str(set_property_i32_template!(field_name).as_str());
        }
    }
    content
}

//todo 注释需要修改
/// return "    this->info = new NativeGroupInfo(info->info);"
fn get_cpp_clz_enum_type_assignment_statement(field_name: &str, p: &GenericPath) -> String {
    let mut content = String::new();
    let name = get_final_name(p.export_name(), p.path().name());
    let clz_name = get_cpp_clz_name(&name);
    match p.ctype() {
        None => {}
        Some(c_type) => match c_type {
            DeclarationType::Union | DeclarationType::Struct => {
                content.push_str(cpp_assignment_statement!(field_name, clz_name).as_str());
            }
            DeclarationType::Enum => {
                content.push_str(cpp_enum_statement!(field_name).as_str());
            }
        },
    }

    content
}

/// return
/// "    napisetPropertyObject(env, val, "info", this->info);"
fn get_cpp_set_property_obj_statement(field_name: &str, p: &GenericPath) -> String {
    let mut content = String::new();
    match p.ctype() {
        None => {
            content.push_str(set_property_obj_template!(field_name).as_str());
        }
        Some(c_type) => match c_type {
            DeclarationType::Union | DeclarationType::Struct => {
                content.push_str(set_property_obj_template!(field_name).as_str());
            }
            DeclarationType::Enum => {
                content.push_str(set_property_i32_template!(field_name).as_str());
            }
        },
    }
    content
}

fn need_build_destruct(p: &GenericPath) -> bool {
    if let Some(c_type) = p.ctype() {
        match c_type {
            DeclarationType::Union | DeclarationType::Struct => true,
            DeclarationType::Enum => false,
        }
    } else {
        false
    }
}

/// return 0:
/// "    napisetPropertyInt32(env, val, "checkTotalCount", this->check_total_count);"
/// "    napiSetPropertyString(env, val, "id", this->id);"
/// "    napisetPropertyBool(env, val, "isRead", this->is_read);"
/// "    napisetPropertyObject(env, val, "info", this->info);"
///
/// 1:"
///     if (this->creator_info != nullptr) {
///         delete this->creator_info;
///     }"
pub fn get_cpp_set_property_on_given_arkts(field: &Field) -> (String, String) {
    let field_name = field.name.as_str();

    let mut content = String::new();

    // println!("field.cfg: {:?}", field.cfg);// 继续全部为none
    //  { annotations: {}, must_use: false, deprecated: None }
    // println!("field.annotations: {:?}", field.annotations);
    let mut need_destruct = false;

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
                    content.push_str(get_cpp_set_property_statement(field_name, &t).as_str());
                }
                Type::Path(p) => {
                    need_destruct = need_build_destruct(&p);
                    content.push_str(get_cpp_set_property_obj_statement(field_name, &p).as_str());
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
                            need_destruct = need_build_destruct(&p);
                            content.push_str(
                                get_cpp_set_property_obj_statement(field_name, &p).as_str(),
                            );
                        }
                        Type::Primitive(primitive_type) => {
                            let t = RPrimitiveType::from(primitive_type);
                            content
                                .push_str(get_cpp_set_property_statement(field_name, &t).as_str());
                        }
                        Type::Array(b_type, constExpr) => {
                            need_destruct = true;
                            println!("");
                        }
                        Type::FuncPtr {
                            ret,          //: Box<Type>
                            args,         //: Vec<(Option<String>, Type)>
                            is_nullable,  //: bool
                            never_return, //: bool
                        } => {
                            need_destruct = true;
                            println!("");
                        }
                    }
                }
                Type::Array(b_type, constExpr) => {
                    need_destruct = true;
                    println!("");
                }
                Type::FuncPtr {
                    ret,          //: Box<Type>
                    args,         //: Vec<(Option<String>, Type)>
                    is_nullable,  //: bool
                    never_return, //: bool
                } => {
                    need_destruct = true;
                    println!("");
                }
            }
        }
        Type::Path(p) => {
            need_destruct = need_build_destruct(&p);
            content.push_str(get_cpp_set_property_obj_statement(field_name, &p).as_str());
        }
        Type::Primitive(primitive_type) => {
            let t = RPrimitiveType::from(primitive_type);
            content.push_str(get_cpp_set_property_statement(field_name, &t).as_str());
        }
        Type::Array(b_type, constExpr) => {
            need_destruct = true;
            match constExpr {
                ConstExpr::Name(name) => {}
                ConstExpr::Value(value) => {}
            }
            content.push_str(format!("NativeList *{};", field_name).as_str());
        }
        Type::FuncPtr {
            ret,          //: Box<Type>,
            args,         //: Vec<(Option<String>, Type)>,
            is_nullable,  //: bool,
            never_return, //: bool,
        } => {
            need_destruct = true;
            content.push_str("Function");
        }
    }

    let destructor_statement: String = if need_destruct {
        let mut destructor_statement = String::from(destructor_statement_template!(field_name));
        destructor_statement
    } else {
        String::new()
    };

    content.push_str("\n");
    (content, destructor_statement)
}

pub(crate) struct CppFieldRet {
    /// std::string user_id;
    pub(crate) member_variable_statement: String,
    /// return this->user_id = charToStringFromSafety(c->user_id);
    pub(crate) variable_assignment_statement: String,
    /// "napisetPropertyInt32(env, val, "checkTotalCount", this->check_total_count);"
    /// "napiSetPropertyString(env, val, "id", this->id);"
    /// "napisetPropertyBool(env, val, "isRead", this->is_read);"
    /// "napisetPropertyObject(env, val, "info", this->info);"
    pub(crate) set_property_statement: String,
    /// if (this->creator_info != nullptr) {
    ///     delete this->creator_info;
    /// }"
    pub(crate) destructor_statement: String,
}

/// 构建出cpp类中每个字段的信息
pub fn build_cpp_field_content(field: &Field) -> CppFieldRet {
    let mut member_variable_statement = get_cpp_parameter_declaration(field);
    let mut variable_assignment_statement = get_cpp_member_variable_statement(&field);

    let (set_property_statement, destructor_statement) = get_cpp_set_property_on_given_arkts(field);
    let mut set_property_statement = set_property_statement;
    let mut destructor_statement = destructor_statement;

    CppFieldRet {
        member_variable_statement,
        variable_assignment_statement,
        set_property_statement,
        destructor_statement,
    }
}
