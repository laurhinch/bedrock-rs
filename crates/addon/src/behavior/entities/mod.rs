pub mod description;

use std::collections::HashMap;

use description::AddonBehaviorEntityDescription;
use serde::{Deserialize, Serialize};

use crate::identifier::AddonIdentifier;

#[derive(Serialize, Deserialize, Debug, Clone)]
/// An Addon behavior entity
pub struct AddonBehaviorEntity {
    /// The format version of the entity
    pub format_version: String,
    /// The definition of the entity (`minecraft:entity`)
    #[serde(rename = "minecraft:entity")]
    pub definition: AddonBehaviorEntityDefinition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// The definition of the entity (`minecraft:entity`)
pub struct AddonBehaviorEntityDefinition {
    /// The description of the entity
    pub description: AddonBehaviorEntityDescription,
    /// The component groups of the entity
    pub component_groups: HashMap<String, serde_json::Value>,
    /// The components of the entity
    pub components: HashMap<AddonIdentifier, serde_json::Value>,
    /// The events of the entity
    pub events: HashMap<String, serde_json::Value>,
}
