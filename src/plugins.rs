//! Provides abstractions and utilities for managing plugins within the SDK, including plugin types, configurations, and versioning.
use thiserror::Error;
use serde::{Serialize, Deserialize};
use url::Url;


#[derive(Debug, Error)]
pub enum PluginError {
    #[cfg(feature = "api")]
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
}

/// used to indicate plugin type & topography.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginVersion {
    pub name: String,
    
    /// when the plugin version was initially created / registered
    pub created_at: i64,
    
    /// the human friendly plugin version.
    pub version: String,

    /// the type of the plugin, indicating its role and topography within the system
    pub plugin_type: String,
    
    /// the hash of the bytes of the plugin's code, used to uniquely identify its version
    /// base64 encoded
    pub hash: String,
    
    /// a signature used to verify the authenticity of the plugin.
    pub signature: Option<String>,
    /// signature algorithm used to sign the plugin, if applicable
    pub sig_alg: Option<String>,
}

impl PluginVersion {
    /// Creates a new instance of `PluginVersion`.
    pub fn new(
        name: String,
        created_at: i64,
        version: String,
        wasm_version: u16,
        plugin_type: String,
        hash: String,
        signature: Option<String>,
        sig_alg: Option<String>,
    ) -> Self {
        Self {
            name,
            created_at,
            version,
            wasm_version,
            plugin_type,
            hash,
            signature,
            sig_alg,
        }
    }

    pub fn read_version(component: &[u8]) -> Result<Option<PluginVersion>, PluginError> {
        // Implementation to fetch plugin version from the given URL
        // This is a placeholder and should be replaced with actual logic

        let parser = Parser::new(0);
        let mut version: Option<PluginVersion> = None;
        for payload in parser.parse_all(&component) {
            match payload? {
                Payload::CustomSection(sec) => {
                    // Handle custom section if needed
                    if sec.name() == "openfab_plugin_version" {
                        let version = Some(postcard::from_bytes(sec.data())?);
                    }
                }
            }
        }
        Ok(version)
    }
}