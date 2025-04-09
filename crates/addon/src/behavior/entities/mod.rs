pub mod description;

use std::collections::HashMap;

use description::AddonEntityDescription;
use serde::{Deserialize, Serialize};

use crate::identifier::AddonIdentifier;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonEntity {
    pub format_version: String,
    #[serde(rename = "minecraft:entity")]
    pub definition: AddonEntityDefinition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonEntityDefinition {
    pub description: AddonEntityDescription,
    pub component_groups: HashMap<String, serde_json::Value>,
    pub components: HashMap<AddonIdentifier, serde_json::Value>,
    pub events: HashMap<String, serde_json::Value>,
}
