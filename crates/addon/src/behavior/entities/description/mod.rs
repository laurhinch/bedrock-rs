pub mod properties;
pub mod scripts;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::identifier::AddonIdentifier;
use self::properties::AddonBehaviorEntityProperty;
use self::scripts::AddonBehaviorEntityScripts;

/// Description of an entity in a Bedrock addon
/// 
/// This contains the fundamental properties that describe an entity
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonBehaviorEntityDescription {
    /// The unique identifier for this entity
    pub identifier: AddonIdentifier,
    /// Controls whether the entity has a spawn egg in the creative UI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_spawnable: Option<bool>,
    /// Controls whether the entity can be summoned using commands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_summonable: Option<bool>,
    /// Controls whether this entity requires experimental gameplay to be enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_experimental: Option<bool>,
    /// Maps internal animation references to actual animations
    /// This is a map of name/animation pairs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<HashMap<String, String>>,
    /// Entity properties for storing dynamic values on the entity
    /// 
    /// # Limitations
    /// - Each entity type is limited to 32 Entity Properties total
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, AddonBehaviorEntityProperty>>,
    /// Maps the entity's animations and animation controllers to run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<AddonBehaviorEntityScripts>,
    /// The Vanilla runtime identifier this entity should use
    /// Generally, the entity will obtain some behavior from this identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_identifier: Option<String>,
    /// The spawn category of the entity
    /// Currently not fully implemented, recommended to leave empty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_category: Option<String>,
}

