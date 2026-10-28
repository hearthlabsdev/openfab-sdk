mod devices;
pub use devices::*;

mod prints;
pub use prints::*;

mod library;
pub use library::*;

mod auth;
pub use auth::*;

mod pricing;
pub use pricing::*;

pub mod error_response;
pub use error_response::*;

pub mod error_response2;
pub use error_response2::*;

pub mod settings;

#[cfg(feature = "plugins")]
mod plugins;
#[cfg(feature = "plugins")]
pub use plugins::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct UploadObject {
    /// the ID of the object created on the server
    pub uid: Uuid,
    /// the upload url to send it to.
    pub put_url: String,
}
