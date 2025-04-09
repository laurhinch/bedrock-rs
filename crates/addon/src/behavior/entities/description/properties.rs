use serde::{Deserialize, Serialize};

/// Default value for a boolean property
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AddonBooleanPropertyDefault {
    /// Boolean value
    Value(bool),
    /// Molang expression
    Expression(String),
}

/// Default value for an integer property
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AddonIntegerPropertyDefault {
    /// Integer value
    Value(i32),
    /// Molang expression
    Expression(String),
}

/// Default value for a float property
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AddonFloatPropertyDefault {
    /// Float value
    Value(f32),
    /// Molang expression
    Expression(String),
}

/// Default value for an enum property
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AddonEnumPropertyDefault {
    /// Enum value
    Value(String),
    /// Molang expression
    Expression(String),
}

/// Entity Properties allow for storing dynamic values on entities.
/// 
/// # Limitations
/// - Each entity type is limited to 32 Entity Properties
/// - Default values can be authored as direct values or Molang expressions
/// - Molang expressions for default values cannot assign or read Molang variables
/// - Only `query.had_component_group` is available in default value expressions
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum AddonBehaviorEntityProperty {
    /// Boolean property type
    /// 
    /// # Default Value
    /// - Must be a boolean value or a Molang expression that returns a numeric value
    /// - Non-zero values are converted to true, zero becomes false
    #[serde(rename = "bool")]
    Bool {
        /// Controls whether the property is synchronized to clients (resource pack)
        /// Default: false
        #[serde(skip_serializing_if = "Option::is_none")]
        client_sync: Option<bool>,
        /// The default value for this property
        default: AddonBooleanPropertyDefault,
    },
    /// Integer property type
    /// 
    /// # Default Value
    /// - Must be an integer or a Molang expression that returns a numeric value
    /// - Result is cast to an integer and clamped to be within the defined range
    #[serde(rename = "int")]
    Int {
        /// Controls whether the property is synchronized to clients (resource pack)
        /// Default: false
        #[serde(skip_serializing_if = "Option::is_none")]
        client_sync: Option<bool>,
        /// The default value for this property
        default: AddonIntegerPropertyDefault,
        /// The [min, max] range for this property
        range: [i32; 2],
    },
    /// Float property type
    /// 
    /// # Default Value
    /// - Must be a float or a Molang expression that returns a numeric value
    /// - Result is cast to a float and clamped to be within the defined range
    #[serde(rename = "float")]
    Float {
        /// Controls whether the property is synchronized to clients (resource pack)
        /// Default: false
        #[serde(skip_serializing_if = "Option::is_none")]
        client_sync: Option<bool>,
        /// The default value for this property
        default: AddonFloatPropertyDefault,
        /// The [min, max] range for this property
        range: [f32; 2],
    },
    /// Enum property type
    /// 
    /// # Limitations
    /// - Limited to a maximum of 16 entries
    /// - Each entry has a minimum length of 1 and maximum of 32 characters
    /// - First character must be alphabetical
    /// - Remaining characters can be alphabetical, numeric, or underscore
    /// 
    /// # Default Value
    /// - Must be a string that matches one of the defined values or a Molang expression
    /// - Molang expressions must return a string that matches one of the defined values
    #[serde(rename = "enum")]
    Enum {
        /// Controls whether the property is synchronized to clients (resource pack)
        /// Default: false
        #[serde(skip_serializing_if = "Option::is_none")]
        client_sync: Option<bool>,
        /// The default value for this property
        default: AddonEnumPropertyDefault,
        /// The list of possible values (maximum 16 entries)
        values: Vec<String>,
    },
} 