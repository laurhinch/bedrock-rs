use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Scripts that define which animations or animation controllers to run
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddonEntityScripts {
    /// The animations and animation controllers to run and their conditions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animate: Option<Vec<AddonEntityScriptAnimation>>,
}

/// Represents animations that can be run in an entity's scripts
/// 
/// Can be either:
/// - A simple string reference to an animation
/// - A conditional object with animation names as keys and Molang conditions as values
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AddonEntityScriptAnimation {
    /// Simple animation name as string
    Simple(String),
    /// Conditional animation with animation name as key and molang condition as value
    Conditional(HashMap<String, String>)
} 