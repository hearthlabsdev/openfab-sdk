use uuid::Uuid;

#[cfg(feature = "fromform")]
use rocket::FromForm;

#[cfg(feature = "schema")]
use rocket_okapi::okapi::{schemars, schemars::JsonSchema};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Brand {
    uuid: Uuid,
    id: Uuid,
    name: String,
    logo_name: Option<String>,
    origin: String,
    slug: String,
}
