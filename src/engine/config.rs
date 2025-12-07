use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Config {}

impl Config {}
