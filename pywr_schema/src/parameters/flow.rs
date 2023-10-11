use crate::parameters::{ParameterMeta, ParameterValueType, ParameterValueTypeMut};
use pywr_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PywrParameter)]
pub struct FlowParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub node: String,
}

impl FlowParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        vec![("node", self.node.as_str())].into_iter().collect()
    }
}
