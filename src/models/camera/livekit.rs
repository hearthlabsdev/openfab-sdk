use serde::{Serialize, Deserialize};

#[cfg(feature = "schema")]
use rocket_okapi::okapi::{schemars, schemars::JsonSchema};

#[cfg(feature = "fromform")]
use rocket::form::FromForm;

/// Used to create a new LiveKit room with the specified name.
/// This struct contains the necessary information to request the creation of a room in LiveKit, primarily the room's name.
#[cfg_attr(feature = "fromform", derive(FromForm))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoom {
    /// Name of the LiveKit room to be created.
    pub name: String,
}
