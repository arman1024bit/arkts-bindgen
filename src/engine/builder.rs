use std::path;
use crate::hbindgen::config::Config;

#[derive(Debug, Clone)]
pub struct Builder {
    config: Config,
}

impl Builder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Builder {
        Builder {
            config: Config::default(),
        }
    }

    #[allow(unused)]
    pub fn with_config(mut self, config: Config) -> Builder {
        self.config = config;
        self
    }
}