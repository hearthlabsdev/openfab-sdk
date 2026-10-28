use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "fromform")]
use rocket::FromForm;

#[cfg(feature = "schema")]
use rocket_okapi::okapi::{schemars, schemars::JsonSchema};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "fromform", derive(FromForm))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct CreateFilament {
    pub name: String,
    pub vendor: Option<String>,
    pub cost: Option<f64>,
    pub colour: String,
    pub density: Option<String>,
    pub spool_weight: Option<String>,
    pub filament_type: Option<String>,
    pub notes: Option<String>,
}
