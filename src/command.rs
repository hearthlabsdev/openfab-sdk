use crate::models::DeviceOrm;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;
use url::Url;
use uuid::Uuid;

use crate::Configuration;

#[derive(Debug, Clone)]
pub struct InnerError {
    /// represents a protocol name such as HTTP, IPP, or any other transport protocol associated with the error.
    pub protocol: Option<String>,
    pub error_code: u32,
    pub description: String,
}

impl fmt::Display for InnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Protocol: {:?}, Error code: {}, Description: {}",
            self.protocol, self.error_code, self.description
        )
    }
}

#[derive(Debug, Error)]
#[error("Command error: {0}")]
pub enum CommandError {
    #[error("Missing parent ID")]
    MissingParentId,

    #[error("Missing unique ID")]
    MissingUniqueId,

    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Serialization error: {0}")]
    SerializationError(InnerError),

    #[error("Deserialization error: {0}")]
    DeserializationError(InnerError),

    #[error("Transport error: {0}")]
    TransportError(InnerError),
}

impl CommandError {
    pub fn from_serialization_error<E: std::fmt::Display>(err: E) -> Self {
        CommandError::SerializationError(InnerError {
            protocol: None,
            error_code: 0,
            description: err.to_string(),
        })
    }

    pub fn from_deserialization_error<E: std::fmt::Display>(err: E) -> Self {
        CommandError::DeserializationError(InnerError {
            protocol: None,
            error_code: 0,
            description: err.to_string(),
        })
    }
}

#[cfg(feature = "api")]
impl From<reqwest::Error> for CommandError {
    fn from(err: reqwest::Error) -> Self {
        let error_code = err.status().map(|s| s.as_u16() as u32).unwrap_or(0);
        CommandError::TransportError(InnerError {
            protocol: Some("HTTP".to_string()),
            error_code,
            description: err.to_string(),
        })
    }
}

pub trait WireFormat: Sized {
    fn encode(&self) -> Result<Vec<u8>, CommandError>;
    fn decode(data: &[u8]) -> Result<Self, CommandError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Json<T>(pub T);

impl<T: Serialize + DeserializeOwned> WireFormat for Json<T> {
    fn encode(&self) -> Result<Vec<u8>, CommandError> {
        serde_json::to_vec(&self.0).map_err(|e| {
            CommandError::SerializationError(InnerError {
                protocol: None,
                error_code: 0,
                description: e.to_string(),
            })
        })
    }

    fn decode(data: &[u8]) -> Result<Self, CommandError> {
        serde_json::from_slice(data)
            .map(|inner| Self(inner))
            .map_err(|e| CommandError::from_deserialization_error(e))
    }
}

#[derive(Debug, Clone)]
pub struct Postcard<T, const N: usize = 1024>(pub T);

impl<T: Serialize + DeserializeOwned, const N: usize> WireFormat for Postcard<T, N> {
    fn encode(&self) -> Result<Vec<u8>, CommandError> {
        Ok(postcard::to_vec::<T, { N }>(&self.0)
            .map_err(|e| CommandError::from_serialization_error(e))?
            .as_slice()
            .to_vec())
    }

    fn decode(data: &[u8]) -> Result<Self, CommandError> {
        postcard::from_bytes(data)
            .map(|inner| Self(inner))
            .map_err(|e| CommandError::from_deserialization_error(e))
    }
}

#[async_trait::async_trait]
pub trait TransportClient<RT: ResourceType + 'static, MT: ModeType + 'static, P: Payload> {
    type Format: WireFormat;
    async fn send(&self, command: Command<RT, MT, P>) -> Result<P::Output, CommandError>;
}

#[cfg(feature = "api")]
#[async_trait::async_trait]
impl<RT: ResourceType + 'static, MT: ModeType + 'static, P: Payload + std::marker::Send>
    TransportClient<RT, MT, P> for reqwest::Client
{
    type Format = Json<Command<RT, MT, P>>;
    async fn send(&self, command: Command<RT, MT, P>) -> Result<P::Output, CommandError> {
        let base = Url::parse("http://localhost").map_err(|e| CommandError::TransportError(InnerError {
            protocol: Some("HTTP".to_string()),
            error_code: 0,
            description: e.to_string(),
        }))?;
        let request = match &Command::<RT, MT, P>::operation().mode {
            ModeEnum::Create => {
                let url = command.route(&base)?;
                log::info!("Sending POST request to URL: {}", url);
                self.request(reqwest::Method::POST, url)
            },
            ModeEnum::List => {
                let url = command.route(&base)?;
                log::info!("Sending GET request to URL: {}", url);
                self.request(reqwest::Method::GET, url)
            },
            ModeEnum::Get => {
                let url = command.route(&base)?;
                log::info!("Sending GET request to URL: {}", url);
                self.request(reqwest::Method::GET, url)
            },
            ModeEnum::Update => {
                let url = command.route(&base)?;
                log::info!("Sending PUT request to URL: {}", url);
                self.request(reqwest::Method::PUT, url)
            },
            ModeEnum::Delete => {
                let url = command.route(&base)?;
                log::info!("Sending DELETE request to URL: {}", url);
                self.request(reqwest::Method::DELETE, url)
            },
        };
        let response = request.send().await.map_err(|e| {
            CommandError::TransportError(InnerError {
                protocol: Some("HTTP".to_string()),
                error_code: 0,
                description: e.to_string(),
            })
        })?;
        let json = response
            .json::<P::Output>()
            .await
            .map_err(|e| CommandError::from_deserialization_error(e))?;
        Ok(json)
    }
}

#[cfg(feature = "api")]
#[async_trait::async_trait]
impl<RT: ResourceType + 'static, MT: ModeType + 'static, P: Payload + std::marker::Send>
    TransportClient<RT, MT, P> for Configuration
{
    type Format = Json<Command<RT, MT, P>>;
    async fn send(&self, command: Command<RT, MT, P>) -> Result<P::Output, CommandError> {
        let base = Url::parse(&self.base_path).map_err(|e| CommandError::TransportError(InnerError {
            protocol: Some("HTTP".to_string()),
            error_code: 0,
            description: e.to_string(),
        }))?;
        log::info!("url: {}", base);
        let request = match &Command::<RT, MT, P>::operation().mode {
            ModeEnum::Create => {
                let url = command.route(&base)?;
                log::info!("Sending POST request to URL: {}", url);
                self.request(reqwest::Method::POST, url)
            },
            ModeEnum::List => {
                let url = command.route(&base)?;
                log::info!("Sending GET request to URL: {}", url);
                self.request(reqwest::Method::GET, url)
            },
            ModeEnum::Get => {
                let url = command.route(&base)?;
                log::info!("Sending GET request to URL: {}", url);
                self.request(reqwest::Method::GET, url)
            },
            ModeEnum::Update => {
                let url = command.route(&base)?;
                log::info!("Sending PUT request to URL: {}", url);
                self.request(reqwest::Method::PUT, url)
            },
            ModeEnum::Delete => {
                let url = command.route(&base)?;
                log::info!("Sending DELETE request to URL: {}", url);
                self.request(reqwest::Method::DELETE, url)
            },
        };
        self.execute(request).await.map_err(|e| {
            CommandError::TransportError(InnerError {
                protocol: Some("HTTP".to_string()),
                error_code: 0,
                description: e.to_string(),
            })
        })
    }
}

pub trait TransportServer<RT: ResourceType + 'static, MT: ModeType + 'static, P: Payload> {
    type CommandType;
    type Format: WireFormat;
    async fn handle(&self, command: Self::CommandType) -> Result<P, CommandError>;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
enum ModeEnum {
    Create = 0,
    List = 1,
    Get = 2,
    Update = 3,
    Delete = 4,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
enum ResourceEnum {
    Device = 0,
    DeviceFiles = 1,
    DeviceConfiguration = 2,
    DeviceLogs = 3,
    DeviceGroups = 4,
    Queue = 5,
    Job = 6,
    Library = 7,
    Asset = 8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Operation {
    pub resource: ResourceEnum,
    pub mode: ModeEnum,
}

impl Operation {
    pub fn new(resource: ResourceEnum, mode: ModeEnum) -> Self {
        Self { resource, mode }
    }

    pub fn as_u16(&self) -> u16 {
        ((self.resource as u16) << 8) | (self.mode as u16)
    }

    pub fn from_u16(value: u16) -> Result<Self, ()> {
        let resource = ((value >> 8) & 0xFF) as u8;
        let mode = (value & 0xFF) as u8;
        let resource_enum = match resource {
            0 => ResourceEnum::Device,
            1 => ResourceEnum::DeviceFiles,
            2 => ResourceEnum::DeviceConfiguration,
            3 => ResourceEnum::DeviceLogs,
            4 => ResourceEnum::DeviceGroups,
            5 => ResourceEnum::Queue,
            6 => ResourceEnum::Job,
            7 => ResourceEnum::Library,
            8 => ResourceEnum::Asset,
            _ => return Err(()),
        };
        let mode_enum = ModeEnum::try_from(mode).map_err(|_| ())?;
        Ok(Self::new(resource_enum, mode_enum))
    }
}

impl TryFrom<u8> for ModeEnum {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ModeEnum::Create),
            1 => Ok(ModeEnum::List),
            2 => Ok(ModeEnum::Get),
            3 => Ok(ModeEnum::Update),
            4 => Ok(ModeEnum::Delete),
            _ => Err(()),
        }
    }
}

// ==========================================
// Generic Marker Traits & ZSTs
// ==========================================

pub trait ResourceType: std::marker::Send {
    const RESOURCE: ResourceEnum;
    fn route() -> &'static str;
}

pub trait ModeType: std::marker::Send {
    const MODE: ModeEnum;
    fn route() -> &'static str;
}

// Resource Markers
#[doc(hidden)]
pub struct Device;
impl ResourceType for Device {
    const RESOURCE: ResourceEnum = ResourceEnum::Device;
    fn route() -> &'static str {
        "/devices"
    }
}

struct Library;
impl ResourceType for Library {
    const RESOURCE: ResourceEnum = ResourceEnum::Library;
    fn route() -> &'static str {
        "/libraries"
    }
}

// Mode Markers
struct Create;
impl ModeType for Create {
    const MODE: ModeEnum = ModeEnum::Create;
    fn route() -> &'static str {
        "/create"
    }
}

#[doc(hidden)]
pub struct List;
impl ModeType for List {
    const MODE: ModeEnum = ModeEnum::List;
    fn route() -> &'static str {
        "/list"
    }
}

struct Get;
impl ModeType for Get {
    const MODE: ModeEnum = ModeEnum::Get;
    fn route() -> &'static str {
        "/<uid>/info"
    }
}

struct Update;
impl ModeType for Update {
    const MODE: ModeEnum = ModeEnum::Update;
    fn route() -> &'static str {
        "/<uid>/update"
    }
}

struct Delete;
impl ModeType for Delete {
    const MODE: ModeEnum = ModeEnum::Delete;
    fn route() -> &'static str {
        "/<uid>/delete"
    }
}

// ==========================================
// Payload & Command Structs
// ==========================================

pub trait Payload: Serialize + for<'de> Deserialize<'de> + 'static + std::marker::Send {
    type Output: for<'de> Deserialize<'de> + 'static;
    fn parent_id(&self) -> Option<Uuid> {
        None
    }
    fn unique_id(&self) -> Option<Uuid> {
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Command<R, M, P> {
    pub payload: P,
    #[serde(skip)]
    _marker: std::marker::PhantomData<(R, M)>,
}

impl<R: ResourceType, M: ModeType, P: Payload> Command<R, M, P> {
    pub fn new(payload: P) -> Self {
        Self {
            payload,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn operation() -> Operation {
        Operation::new(R::RESOURCE, M::MODE)
    }

    pub fn route(&self, base: &Url) -> Result<Url, CommandError> {
        let base_route = format!("/api/{}{}", R::route(), M::route());
        log::info!("base_route: {}", base_route);
        let route = match base_route.contains("<uid>") {
            true => base_route.replace(
                "<uid>",
                &self
                    .payload
                    .unique_id()
                    .ok_or(CommandError::MissingUniqueId)?
                    .to_string(),
            ),
            false => base_route,
        };
        let route = match route.contains("<pid>") {
            true => route.replace(
                "<pid>",
                &self
                    .payload
                    .parent_id()
                    .ok_or(CommandError::MissingParentId)?
                    .to_string(),
            ),
            false => route,
        };
        Ok(base.join(&route)?)
    }
}

impl<R: ResourceType + 'static, M: ModeType + 'static, P: Payload> Command<R, M, P> {
    pub async fn run(
        self,
        client: &impl TransportClient<R, M, P>,
    ) -> Result<P::Output, CommandError>
    where
        for<'de> P::Output: Deserialize<'de> + 'static,
    {
        client.send(self).await
    }
}

// ==========================================
// Example Usage & Type Aliases
// ==========================================

#[doc(hidden)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDevicesPayload {
    pub group: Option<Uuid>,
}

pub struct ListPayload<O: Deserialize<'static>> {
    pub parent: Option<Uuid>,
    output: std::marker::PhantomData<O>,
}

impl From<Option<Uuid>> for ListDevicesPayload {
    fn from(group: Option<Uuid>) -> Self {
        Self { group }
    }
}

impl From<Uuid> for ListDevicesPayload {
    fn from(group: Uuid) -> Self {
        Self { group: Some(group) }
    }
}

impl Payload for ListDevicesPayload {
    type Output = Vec<DeviceOrm>;
    fn parent_id(&self) -> Option<Uuid> {
        self.group
    }
}

/// Type alias completely determining the operation statically
pub type ListDevices = Command<Device, List, ListDevicesPayload>;
