//! This module declares the models used to handle plugin orchestration.

use serde::{Serialize, Deserialize};

#[cfg(feature = "schema")]
use rocket_okapi::okapi::{schemars, schemars::JsonSchema};

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PluginProfile {
    pub name: String,
    /// the entry point for the plugin.
    pub main: String,
    /// called when the plugin is loaded
    pub load: Option<String>,
    /// called when the plugin is unloaded
    pub unload: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Plugin {
    pub profile: PluginProfile,
    pub name: String,
    
}