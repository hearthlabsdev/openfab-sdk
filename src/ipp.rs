//! # OpenFAB IPP Compatibility & Extension Layer
//!
//! This module provides a compatibility layer between the Internet Printing Protocol (IPP)
//! and the OpenFAB API, translating IPP attributes into OpenFAB-compatible resource representations.
//! It serves as an experimental testbed for machine-agnostic manufacturing workflows and
//! extensions to the IPP 3D specification (PWG 5100.21).
//!
//! ## Architectural Rationale: Why Extend IPP?
//!
//! Small-scale and distributed manufacturing suffers from severe vendor lock-in, fragmented
//! 3MF metadata across vendors (Prusa, Bambu, Creality, etc.), and a complete lack of
//! unified programmatic control for diverse fabrication equipment (CNC routers, laser cutters,
//! waterjets, and additive machinery).
//!
//! ## Abstraction Over Hardware-Specific Structure (Drivers)
//!
//! By introducing an abstraction layer over IPP, this design frees client implementations
//! from hardware-specific implementation details. Clients only need to reason about high-level
//! operational capabilities—such as **dimensionality (2D vs. 3D)**, **power profiles**, **feed/stitch rates**,
//! and **build volumes**—rather than bespoke machine commands.
//!
//! ## Server-Side Drivers & Third-Party Extensibility
//!
//! While standard IPP eliminates *client-side* drivers for end users, managing a diverse
//! manufacturing farm requires tracking **server-side device middleware** that translates
//! standard IPP requests into machine-specific instructions.
//!
//! By allowing third-party developers to author their own identifiable compatibility layers,
//! this approach lowers the barrier to entry for hardware manufacturers and open-source
//! contributors alike, turning bespoke device support into a modular, community-driven ecosystem.
//! This also provides a mechanism to monitor, and replace faulty or outdated server-side drivers, ensuring that the manufacturing ecosystem remains robust, up-to-date, and capable of integrating new hardware seamlessly, without imposing additional complexity on client implementations.
//!
//! ## Proposed Extensions and Attributes
//!
//! 1.0 **Server-Side Driver Identification**
//!
//! | Section | Attribute | Description |
//! |-----------|-------------|
//! | 1.1 | `driver-id` | Specifies the unique identifier of the server-side driver. |
//! | 1.2 | `driver-version` | Specifies the version of the server-side driver. |
//! | 1.3 | `driver-fingerprint` | Specifies the fingerprint of the server-side driver. |
//! | 1.4 | `driver-name` | Specifies the human readable name of the driver. |
//!
//! 1.1 **Driver ID**
//!
//! This attribute specifies the unique identifier of the server-side driver responsible for translating standard IPP requests into machine-specific instructions. It allows clients and administrators to uniquely identify which middleware component is handling a particular device, facilitating monitoring, debugging, and updates.
//!
//! Specification on how it should be derived or assigned will come later.
//!
//! 1.2 **Driver Version**
//!
//! This attribute specifies the version of the server-side driver responsible for translating standard IPP requests into machine-specific instructions. It allows clients and administrators to track which version of the middleware component is handling a particular device, facilitating monitoring, debugging, and updates.
//!
//! 1.3 **Driver Fingerprint**
//!
//! A fingerprint is a way to ensure the authenticity of the driver, this should be a signed hash of the driver code.
//! It allows clients and administrators to verify that the driver has not been tampered with and is indeed the expected version, providing an additional layer of security and trust to prevent unauthorized or malicious modifications.
//!
//! 1.4 **Driver Name**
//!
//! This attribute should be a human readable name.
//!
use crate::models::CreateDevice;
use ipp::attribute::{IppAttribute, IppAttributeName, IppAttributes};
use ipp::model::DelimiterTag;
use ipp::value::IppValue;
use std::collections::HashMap;
use thiserror::Error;

pub fn get_attribute<'a>(
    attrs: &'a IppAttributes,
    tag: DelimiterTag,
    name: &'static str,
) -> Result<&'a IppValue, IppConversionError> {
    Ok(attrs
        .groups()
        .iter()
        .filter(|g| g.tag() == tag)
        .filter_map(|g| g.get(name))
        .next()
        .ok_or(IppConversionError::MissingAttribute(name))?
        .value())
}

pub fn get_name(
    attrs: &IppAttributes,
    tag: DelimiterTag,
    attr_name: &'static str,
) -> Result<String, IppConversionError> {
    match get_attribute(attrs, tag, attr_name)? {
        IppValue::NameWithLanguage { name, language: _ } => Ok(name.as_str().to_string()),
        IppValue::NameWithoutLanguage(name) => Ok(name.as_str().to_string()),
        _ => Err(IppConversionError::WrongAttributeType(
            attr_name,
            "NameWithLanguage or NameWithoutLanguage".into(),
        )),
    }
}

pub fn get_text(
    attrs: &IppAttributes,
    tag: DelimiterTag,
    attr_name: &'static str,
) -> Result<String, IppConversionError> {
    match get_attribute(attrs, tag, attr_name)? {
        IppValue::TextWithLanguage { text, language: _ } => Ok(text.to_string()),
        IppValue::TextWithoutLanguage(text) => Ok(text.to_string()),
        _ => Err(IppConversionError::WrongAttributeType(
            attr_name,
            "TextWithLanguage or TextWithoutLanguage".into(),
        )),
    }
}

#[derive(Debug, Error)]
pub enum IppConversionError {
    #[error("Missing required attribute: {0}")]
    MissingAttribute(&'static str),
    #[error("Attribute has wrong type: {0}, expected {1}")]
    WrongAttributeType(&'static str, String),
    #[error("Failed to parse IPP attributes: {0}")]
    IppParseError(#[from] ipp::parser::IppParseError),
}

impl TryFrom<IppAttributes> for CreateDevice {
    type Error = IppConversionError;
    fn try_from(value: IppAttributes) -> Result<Self, IppConversionError> {
        Ok(Self {
            name: get_name(
                &value,
                DelimiterTag::PrinterAttributes,
                &IppAttributeName::PRINTER_NAME,
            )?
            .as_str()
            .to_string(),
            driver_version: get_text(&value, DelimiterTag::PrinterAttributes, "driver-version")?
                .as_str()
                .to_string(),
            driver_id: get_text(&value, DelimiterTag::PrinterAttributes, "driver-id")?
                .as_str()
                .to_string(),
            enabled: true,
            driver_extras: None,
            upload_dir: None,
            serial: "".to_string(),
            status: get_text(
                &value,
                DelimiterTag::PrinterAttributes,
                &IppAttributeName::PRINTER_STATE,
            )?
            .as_str()
            .to_string(),
        })
    }
}
/*
impl TryInto<IppAttributes> for CreateDevice {
    type Error = IppConversionError;
    fn try_into(self) -> Result<IppAttributes, IppConversionError> {
        let mut attrs = IppAttributes::new();
        attrs.insert(
            DelimiterTag::PrinterAttributes,
            IppAttributeName::PRINTER_NAME,
            IppValue::NameWithoutLanguage(self.name),
        );
        attrs.add(
            DelimiterTag::PrinterAttributes,
            IppAttribute::with_name("driver-version", IppValue::TextWithoutLanguage(self.driver_version))?,
        );
        attrs.add(
            DelimiterTag::PrinterAttributes,
            IppAttribute::with_name("driver-id", IppValue::TextWithoutLanguage(self.driver_id))?,
        );
        attrs.add(
            DelimiterTag::PrinterAttributes,
            IppAttribute::with_name(IppAttributeName::PRINTER_STATE, IppValue::TextWithoutLanguage(self.status))?,
        );
        Ok(attrs)
    }
}*/
