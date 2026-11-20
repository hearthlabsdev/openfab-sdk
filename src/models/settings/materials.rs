use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[cfg(feature = "fromform")]
use rocket::FromForm;

#[cfg(feature = "schema")]
use rocket_okapi::okapi::{schemars, schemars::JsonSchema};

use std::collections::HashMap;

#[cfg_attr(feature = "fromform", derive(FromFormField))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum MaterialDB {
    #[default]
    Internal,
    OpenFilamentDB,
    External(String),
}

impl fmt::Display for MaterialDB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaterialDB::Internal => write!(f, "internal"),
            MaterialDB::OpenFilamentDB => write!(f, "openfilamentdb"),
            MaterialDB::External(s) => write!(f, "external({})", s),
        }
    }
}

impl FromStr for MaterialDB {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "internal" => Ok(MaterialDB::Internal),
            "openfilamentdb" => Ok(MaterialDB::OpenFilamentDB),
            s if s.starts_with("external(") && s.ends_with(")") => {
                Ok(MaterialDB::External(s[9..s.len() - 1].to_string()))
            }
            _ => Ok(MaterialDB::External(s.to_string())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Material {
    pub uuid: Uuid,
    pub material: String,
    pub material_class: String,
    pub id: String,
    pub brand_id: Uuid,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Filament {
    pub uid: Uuid,
    pub source_id: Uuid,
    pub source: MaterialDB,
    pub name: String,
    pub diameter_tolerance: f64,
    pub density: f64,
    pub min_primt_temperature: u32,
    pub max_print_temperature: u32,
    pub min_bed_temperature: u32,
    pub max_bed_temperature: u32,
    pub discontinued: bool,
    pub brand_id: Uuid,
    pub material_id: Uuid,
    pub slug: String,
    pub material: String,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct FilamentVariant {
    pub uid: Uuid,
    pub source_id: Uuid,
    pub source: MaterialDB,
    pub name: String,
    pub color_hex: String,
    pub discontinued: bool,
    pub filament_id: Uuid,
    pub slug: String,
    pub traits: Option<HashMap<String, bool>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct FilamentSize {
    pub uid: Uuid,
    pub source_id: Uuid,
    pub source: MaterialDB,
    pub filament_weight: f64,
    pub diameter: f64,
    pub spool_refill: bool,
    pub empty_spool_weight: bool,
    pub spool_core_diameter: f64,
    pub article_number: String,
    pub discontinued: bool,
    pub id: Uuid,
    pub variant_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Store {
    pub uid: Uuid,
    pub source_id: Uuid,
    pub name: String,
    pub source: MaterialDB,
    pub storefront_url: String,
    pub logo_name: String,
    /// country code that this store ships to
    pub ships_from: Option<Vec<String>>,
    /// country code that this store ships to
    pub ships_to: Option<Vec<String>>,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PurchaseURL {
    pub store_id: Uuid,
    pub url: String,
    pub uid: Uuid,
    pub source_id: Uuid,
    pub source: MaterialDB,
    pub size_id: Uuid,
    pub spool_refill: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct CreateMaterial {
    pub material: String,
    pub material_class: String,
    pub id: String,
    pub brand_id: Uuid,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "fromform", derive(FromForm))]
pub struct CreateFilament {
    pub source_id: Uuid,
    pub source: Option<MaterialDB>,
    pub name: String,
    pub diameter_tolerance: f64,
    pub density: f64,
    pub min_primt_temperature: u32,
    pub max_print_temperature: u32,
    pub min_bed_temperature: u32,
    pub max_bed_temperature: u32,
    #[serde(default)]
    pub discontinued: bool,
    pub brand_id: Uuid,
    pub material_id: Uuid,
    pub slug: String,
    pub material: String,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "fromform", derive(FromForm))]
pub struct CreateFilamentVariant {
    pub source_id: Uuid,
    pub source: Option<MaterialDB>,
    pub name: String,
    pub color_hex: String,
    #[serde(default)]
    pub discontinued: bool,
    pub filament_id: Uuid,
    pub slug: String,
    pub traits: Option<HashMap<String, bool>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "fromform", derive(FromForm))]
pub struct CreateFilamentSize {
    pub source_id: Uuid,
    pub source: Option<MaterialDB>,
    pub filament_weight: f64,
    pub diameter: f64,
    pub spool_refill: bool,
    pub empty_spool_weight: bool,
    pub spool_core_diameter: f64,
    pub article_number: String,
    #[serde(default)]
    pub discontinued: bool,
    pub id: Uuid,
    pub variant_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "fromform", derive(FromForm))]
pub struct CreateStore {
    pub source_id: Uuid,
    pub source: Option<MaterialDB>,
    pub name: String,
    pub storefront_url: String,
    pub logo_name: String,
    /// Country codes that this store ships from
    pub ships_from: Option<Vec<String>>,
    /// Country codes that this store ships to
    pub ships_to: Option<Vec<String>>,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "fromform", derive(FromForm))]
pub struct CreatePurchaseURL {
    pub source_id: Uuid,
    pub source: Option<MaterialDB>,
    pub store_id: Uuid,
    pub url: String,
    pub size_id: Uuid,
    pub refill: bool,
}
