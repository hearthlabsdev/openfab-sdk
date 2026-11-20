use crate::models::DeviceOrm;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Error)]
#[error("Command error: {0}")]
pub enum CommandError {
    #[error("Missing parent ID")]
    MissingParentId,
    #[error("Missing unique ID")]
    MissingUniqueId,
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
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

pub trait ResourceType {
    const RESOURCE: ResourceEnum;
    fn route() -> &'static str;
}

pub trait ModeType {
    const MODE: ModeEnum;
    fn route() -> &'static str;
}

// Resource Markers
struct Device;
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

struct List;
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

pub trait Payload: Serialize + for<'de> Deserialize<'de> {
    type Output;
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

    pub fn route(&self) -> Result<Url, CommandError> {
        let base_route = format!("{}{}", R::route(), M::route());
        let route = base_route
            .replace(
                "<uid>",
                &self
                    .payload
                    .unique_id()
                    .ok_or(CommandError::MissingUniqueId)?
                    .to_string(),
            )
            .replace(
                "<pid>",
                &self
                    .payload
                    .parent_id()
                    .ok_or(CommandError::MissingParentId)?
                    .to_string(),
            );
        Ok(Url::parse(&route)?)
    }

    pub fn run(self) -> Result<P::Output, CommandError> {
        // Dispatch / execution hook
        todo!()
    }
}

// ==========================================
// Example Usage & Type Aliases
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ListDevicesPayload {
    pub group: Option<Uuid>,
}

impl Payload for ListDevicesPayload {
    type Output = Vec<DeviceOrm>;
    fn parent_id(&self) -> Option<Uuid> {
        self.group
    }
}

/// Type alias completely determining the operation statically
pub type ListDevicesCommand = Command<Device, List, ListDevicesPayload>;
