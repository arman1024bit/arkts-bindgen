use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

pub type ExportFunctionNameListener = Box<dyn Fn(&str) -> String + Send + Sync + 'static>;

lazy_static! {
    pub(crate) static ref GLOBAL_EXPORT_FN_NAME_LISTENER: Arc<Mutex<Option<ExportFunctionNameListener>>> =
        Arc::new(Mutex::new(None));
}

pub fn set_export_fn_name_listener(listener_opt: Option<ExportFunctionNameListener>) {
    match GLOBAL_EXPORT_FN_NAME_LISTENER.lock() {
        Ok(mut my_listener) => match listener_opt {
            None => {
                *my_listener = None;
            }
            Some(lsr) => {
                *my_listener = Some(lsr);
            }
        },
        Err(error) => {}
    }
}

pub(crate) fn on_export_fn_name_listener(fun_name: &str) -> String {
    match GLOBAL_EXPORT_FN_NAME_LISTENER.lock() {
        Ok(listener_opt) => {
            if let Some(listener) = &*listener_opt {
                return listener(fun_name);
            }
            fun_name.to_string()
        }
        Err(_error) => fun_name.to_string(),
    }
}
