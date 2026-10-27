use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[cfg(feature = "fromform")]
use rocket::FromForm;

#[cfg(feature = "schema")]
use rocket_okapi::okapi::{schemars, schemars::JsonSchema};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "fromform", derive(FromForm))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct CreateDeviceModel {
    pub name: String,
    // Comma-separated or parsed fields depending on form handling, or use standard form types
    pub technology: Option<String>,
    pub family: Option<String>,
    pub thumbnail: Option<Uuid>,
    pub bed_texture: Option<Uuid>,
    pub bed_model: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct DeviceModelInfo {
    pub uid: Uuid,
    pub name: String,
    pub variants: Vec<String>,
    pub technology: Option<String>,
    pub family: Option<String>,
    pub url: Option<String>,
    pub default_materials: Vec<String>,
}