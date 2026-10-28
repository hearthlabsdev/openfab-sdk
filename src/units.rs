//! A simplification for handling unit definitions
//! at some point I'd like to switch this to use the seven exponents codes as u128 constants
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;
use std::collections::HashMap;

#[cfg(feature = "schema")]
use rocket_okapi::okapi::{schemars, schemars::JsonSchema};

#[cfg(feature = "fromform")]
use rocket::FromFormField;

// ==========================================
// TimeUnit
// ==========================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "fromform", derive(FromFormField))]
pub enum TimeUnit {
    #[default]
    Second,
    Minute,
    Hour,
}

impl TimeUnit {
    pub const fn list() -> &'static [TimeUnit] {
        &[TimeUnit::Second, TimeUnit::Minute, Self::Hour]
    }
    pub const fn fullname(&self) -> &'static str {
        match self {
            Self::Second => "second",
            Self::Minute => "minute",
            Self::Hour => "hour",
        }
    }
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Second => write!(f, "s"),
            Self::Minute => write!(f, "min"),
            Self::Hour => write!(f, "h"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseTimeUnitError(pub String);

impl fmt::Display for ParseTimeUnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid time unit: '{}'. Expected s, min, or h (and variants)",
            self.0
        )
    }
}

impl std::error::Error for ParseTimeUnitError {}

impl FromStr for TimeUnit {
    type Err = ParseTimeUnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "s" | "sec" | "second" | "seconds" => Ok(Self::Second),
            "min" | "minute" | "minutes" => Ok(Self::Minute),
            "h" | "hr" | "hour" | "hours" => Ok(Self::Hour),
            _ => Err(ParseTimeUnitError(s.to_string())),
        }
    }
}

// ==========================================
// LengthUnit
// ==========================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "fromform", derive(FromFormField))]
pub enum LengthUnit {
    Millimeter,
    Centimeter,
    #[default]
    Meter,
    Kilometer,
}

impl LengthUnit {
    pub const fn list() -> &'static [LengthUnit] {
        &[LengthUnit::Millimeter, LengthUnit::Centimeter, LengthUnit::Meter, LengthUnit::Kilometer]
    }
    pub const fn fullname(&self) -> &'static str {
        match self {
            Self::Millimeter => "millimeter",
            Self::Centimeter => "centimeter",
            Self::Meter => "meter",
            Self::Kilometer => "kilometer",
        }
    }
}

impl fmt::Display for LengthUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Millimeter => write!(f, "mm"),
            Self::Centimeter => write!(f, "cm"),
            Self::Meter => write!(f, "m"),
            Self::Kilometer => write!(f, "km"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseLengthUnitError(pub String);

impl fmt::Display for ParseLengthUnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid length unit: '{}'. Expected mm, cm, m, or km",
            self.0
        )
    }
}

impl std::error::Error for ParseLengthUnitError {}

impl FromStr for LengthUnit {
    type Err = ParseLengthUnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "mm" | "millimeter" | "millimeters" => Ok(Self::Millimeter),
            "cm" | "centimeter" | "centimeters" => Ok(Self::Centimeter),
            "m" | "meter" | "meters" => Ok(Self::Meter),
            "km" | "kilometer" | "kilometers" => Ok(Self::Kilometer),
            _ => Err(ParseLengthUnitError(s.to_string())),
        }
    }
}

// ==========================================
// VolumeUnit
// ==========================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "fromform", derive(FromFormField))]
pub enum VolumeUnit {
    Milliliter,
    #[default]
    Liter,
}

impl VolumeUnit {
    pub const fn list() -> &'static [VolumeUnit] {
        &[Self::Milliliter, Self::Liter]
    }
    pub const fn fullname(&self) -> &'static str {
        match self {
            Self::Milliliter => "milliliter",
            Self::Liter => "liter",
        }
    }
}

impl fmt::Display for VolumeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Milliliter => write!(f, "mL"),
            Self::Liter => write!(f, "L"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseVolumeUnitError(pub String);

impl fmt::Display for ParseVolumeUnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid volume unit: '{}'. Expected mL or L", self.0)
    }
}

impl std::error::Error for ParseVolumeUnitError {}

impl FromStr for VolumeUnit {
    type Err = ParseVolumeUnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "ml" | "milliliter" | "milliliters" => Ok(Self::Milliliter),
            "l" | "liter" | "liters" => Ok(Self::Liter),
            _ => Err(ParseVolumeUnitError(s.to_string())),
        }
    }
}

// ==========================================
// WeightUnit
// ==========================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "fromform", derive(FromFormField))]
pub enum WeightUnit {
    Milligram,
    Gram,
    #[default]
    Kilogram,
}

impl WeightUnit {
    pub const fn list() -> &'static [WeightUnit] {
        &[Self::Milligram, Self::Gram, Self::Kilogram]
    }
    pub const fn fullname(&self) -> &'static str {
        match self {
            Self::Milligram => "milligram",
            Self::Gram => "gram",
            Self::Kilogram => "kilogram",
        }
    }
}

impl fmt::Display for WeightUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Milligram => write!(f, "mg"),
            Self::Gram => write!(f, "g"),
            Self::Kilogram => write!(f, "kg"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseWeightUnitError(pub String);

impl fmt::Display for ParseWeightUnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid weight unit: '{}'. Expected mg, g, or kg",
            self.0
        )
    }
}

impl std::error::Error for ParseWeightUnitError {}

impl FromStr for WeightUnit {
    type Err = ParseWeightUnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "mg" | "milligram" | "milligrams" => Ok(Self::Milligram),
            "g" | "gram" | "grams" => Ok(Self::Gram),
            "kg" | "kilogram" | "kilograms" => Ok(Self::Kilogram),
            _ => Err(ParseWeightUnitError(s.to_string())),
        }
    }
}

// ==========================================
// AreaUnit
// ==========================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "fromform", derive(FromFormField))]
pub enum AreaUnit {
    SquareMillimeter,
    SquareCentimeter,
    #[default]
    SquareMeter,
}

impl AreaUnit {
    pub const fn list() -> &'static [AreaUnit] {
        &[Self::SquareMillimeter, Self::SquareCentimeter, Self::SquareMeter]
    }
    pub const fn fullname(&self) -> &'static str {
        match self {
            Self::SquareMillimeter => "square millimeter",
            Self::SquareCentimeter => "square centimeter",
            Self::SquareMeter => "square meter",
        }
    }
}

impl fmt::Display for AreaUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SquareMillimeter => write!(f, "mm²"),
            Self::SquareCentimeter => write!(f, "cm²"),
            Self::SquareMeter => write!(f, "m²"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseAreaUnitError(pub String);

impl fmt::Display for ParseAreaUnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid area unit: '{}'. Expected mm², cm², or m²",
            self.0
        )
    }
}

impl std::error::Error for ParseAreaUnitError {}

impl FromStr for AreaUnit {
    type Err = ParseAreaUnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_lowercase();
        match normalized.as_str() {
            "mm2" | "mm²" | "sqmm" | "square millimeter" | "square millimeters" => {
                Ok(Self::SquareMillimeter)
            }
            "cm2" | "cm²" | "sqcm" | "square centimeter" | "square centimeters" => {
                Ok(Self::SquareCentimeter)
            }
            "m2" | "m²" | "sqm" | "square meter" | "square meters" => Ok(Self::SquareMeter),
            _ => Err(ParseAreaUnitError(s.to_string())),
        }
    }
}

// ==========================================
// PowerUnit
// ==========================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "fromform", derive(FromFormField))]
pub enum PowerUnit {
    Milliwatt,
    #[default]
    Watt,
    Kilowatt,
    Megawatt,
}

impl PowerUnit {
    pub const fn list() -> &'static [PowerUnit] {
        &[Self::Milliwatt, Self::Watt, Self::Kilowatt, Self::Megawatt]
    }
    pub const fn fullname(&self) -> &'static str {
        match self {
            Self::Milliwatt => "milliwatt",
            Self::Watt => "watt",
            Self::Kilowatt => "kilowatt",
            Self::Megawatt => "megawatt",
        }
    }
}

impl fmt::Display for PowerUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Milliwatt => write!(f, "mW"),
            Self::Watt => write!(f, "W"),
            Self::Kilowatt => write!(f, "kW"),
            Self::Megawatt => write!(f, "MW"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsePowerUnitError(pub String);

impl fmt::Display for ParsePowerUnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid power unit: '{}'. Expected mW, W, kW, or MW",
            self.0
        )
    }
}

impl std::error::Error for ParsePowerUnitError {}

impl FromStr for PowerUnit {
    type Err = ParsePowerUnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_string();
        match normalized.as_str() {
            "mw" | "milliwatt" | "milliwatts" => Ok(Self::Milliwatt),
            "w" | "watt" | "watts" => Ok(Self::Watt),
            "kw" | "kilowatt" | "kilowatts" => Ok(Self::Kilowatt),
            _ => {
                if s.trim() == "MW" {
                    Ok(Self::Megawatt)
                } else {
                    Err(ParsePowerUnitError(s.to_string()))
                }
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseUnitError {
    #[error("unrecognized time unit: {0}")]
    Time(#[from] ParseTimeUnitError),

    #[error("unrecognized length unit: {0}")]
    Length(#[from] ParseLengthUnitError),

    #[error("unrecognized volume unit: {0}")]
    Volume(#[from] ParseVolumeUnitError),

    #[error("unrecognized weight unit: {0}")]
    Weight(#[from] ParseWeightUnitError),

    #[error("unrecognized area unit: {0}")]
    Area(#[from] ParseAreaUnitError),

    #[error("unrecognized power unit: {0}")]
    Power(#[from] ParsePowerUnitError),
}

// ==========================================
// Unit Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_unit_roundtrip_and_fullnames() {
        let cases = [
            (
                TimeUnit::Second,
                "s",
                "second",
                vec!["s", "sec", "second", "seconds"],
            ),
            (
                TimeUnit::Minute,
                "min",
                "minute",
                vec!["min", "minute", "minutes"],
            ),
            (
                TimeUnit::Hour,
                "h",
                "hour",
                vec!["h", "hr", "hour", "hours"],
            ),
        ];

        for (unit, symbol, fullname, aliases) in cases {
            assert_eq!(unit.fullname(), fullname);
            assert_eq!(unit.to_string(), symbol);

            for alias in aliases {
                let parsed = TimeUnit::from_str(alias).unwrap();
                assert_eq!(parsed, unit);
                // Round-trip check: Display output parsed back should equal the unit
                let displayed = parsed.to_string();
                let roundtripped = TimeUnit::from_str(&displayed).unwrap();
                assert_eq!(roundtripped, unit);
            }
        }

        assert!(TimeUnit::from_str("invalid_time").is_err());
    }

    #[test]
    fn test_length_unit_roundtrip_and_fullnames() {
        let cases = [
            (
                LengthUnit::Millimeter,
                "mm",
                "millimeter",
                vec!["mm", "millimeter", "millimeters"],
            ),
            (
                LengthUnit::Centimeter,
                "cm",
                "centimeter",
                vec!["cm", "centimeter", "centimeters"],
            ),
            (
                LengthUnit::Meter,
                "m",
                "meter",
                vec!["m", "meter", "meters"],
            ),
            (
                LengthUnit::Kilometer,
                "km",
                "kilometer",
                vec!["km", "kilometer", "kilometers"],
            ),
        ];

        for (unit, symbol, fullname, aliases) in cases {
            assert_eq!(unit.fullname(), fullname);
            assert_eq!(unit.to_string(), symbol);

            for alias in aliases {
                let parsed = LengthUnit::from_str(alias).unwrap();
                assert_eq!(parsed, unit);
                let roundtripped = LengthUnit::from_str(&parsed.to_string()).unwrap();
                assert_eq!(roundtripped, unit);
            }
        }

        assert!(LengthUnit::from_str("lightyear").is_err());
    }

    #[test]
    fn test_volume_unit_roundtrip_and_fullnames() {
        let cases = [
            (
                VolumeUnit::Milliliter,
                "mL",
                "milliliter",
                vec!["ml", "milliliter", "milliliters"],
            ),
            (
                VolumeUnit::Liter,
                "L",
                "liter",
                vec!["l", "liter", "liters"],
            ),
        ];

        for (unit, symbol, fullname, aliases) in cases {
            assert_eq!(unit.fullname(), fullname);
            assert_eq!(unit.to_string(), symbol);

            for alias in aliases {
                let parsed = VolumeUnit::from_str(alias).unwrap();
                assert_eq!(parsed, unit);
                let roundtripped = VolumeUnit::from_str(&parsed.to_string()).unwrap();
                assert_eq!(roundtripped, unit);
            }
        }

        assert!(VolumeUnit::from_str("gallon").is_err());
    }

    #[test]
    fn test_weight_unit_roundtrip_and_fullnames() {
        let cases = [
            (
                WeightUnit::Milligram,
                "mg",
                "milligram",
                vec!["mg", "milligram", "milligrams"],
            ),
            (WeightUnit::Gram, "g", "gram", vec!["g", "gram", "grams"]),
            (
                WeightUnit::Kilogram,
                "kg",
                "kilogram",
                vec!["kg", "kilogram", "kilograms"],
            ),
        ];

        for (unit, symbol, fullname, aliases) in cases {
            assert_eq!(unit.fullname(), fullname);
            assert_eq!(unit.to_string(), symbol);

            for alias in aliases {
                let parsed = WeightUnit::from_str(alias).unwrap();
                assert_eq!(parsed, unit);
                let roundtripped = WeightUnit::from_str(&parsed.to_string()).unwrap();
                assert_eq!(roundtripped, unit);
            }
        }

        assert!(WeightUnit::from_str("pound").is_err());
    }

    #[test]
    fn test_area_unit_roundtrip_and_fullnames() {
        let cases = [
            (
                AreaUnit::SquareMillimeter,
                "mm²",
                "square millimeter",
                vec!["mm2", "mm²", "sqmm", "square millimeter"],
            ),
            (
                AreaUnit::SquareCentimeter,
                "cm²",
                "square centimeter",
                vec!["cm2", "cm²", "sqcm", "square centimeter"],
            ),
            (
                AreaUnit::SquareMeter,
                "m²",
                "square meter",
                vec!["m2", "m²", "sqm", "square meter"],
            ),
        ];

        for (unit, symbol, fullname, aliases) in cases {
            assert_eq!(unit.fullname(), fullname);
            assert_eq!(unit.to_string(), symbol);

            for alias in aliases {
                let parsed = AreaUnit::from_str(alias).unwrap();
                assert_eq!(parsed, unit);
                let roundtripped = AreaUnit::from_str(&parsed.to_string()).unwrap();
                assert_eq!(roundtripped, unit);
            }
        }

        assert!(AreaUnit::from_str("acre").is_err());
    }

    #[test]
    fn test_power_unit_roundtrip_and_fullnames() {
        let cases = [
            (
                PowerUnit::Milliwatt,
                "mW",
                "milliwatt",
                vec!["mw", "milliwatt", "milliwatts"],
            ),
            (PowerUnit::Watt, "W", "watt", vec!["w", "watt", "watts"]),
            (
                PowerUnit::Kilowatt,
                "kW",
                "kilowatt",
                vec!["kw", "kilowatt", "kilowatts"],
            ),
            (PowerUnit::Megawatt, "MW", "megawatt", vec!["MW"]),
        ];

        for (unit, symbol, fullname, aliases) in cases {
            assert_eq!(unit.fullname(), fullname);
            assert_eq!(unit.to_string(), symbol);

            for alias in aliases {
                let parsed = PowerUnit::from_str(alias).unwrap();
                assert_eq!(parsed, unit);
            }
        }

        assert!(PowerUnit::from_str("horsepower").is_err());
    }
}

pub struct UnitEntry {
    pub name: &'static str,
    pub entries: Vec<String>,
}
pub fn unit_list() -> HashMap<String, Vec<String>> {
    let mut units = HashMap::new();

    let time = TimeUnit::list().iter().map(|v| v.fullname().to_string()).collect::<Vec<String>>();
    units.insert("time".into(), time);

    let length = LengthUnit::list().iter().map(|v| v.fullname().to_string()).collect::<Vec<String>>();
    units.insert("length".into(), length);

    let area = AreaUnit::list().iter().map(|v| v.fullname().to_string()).collect::<Vec<String>>();
    units.insert("area".into(), area);

    let volume = VolumeUnit::list().iter().map(|v| v.fullname().to_string()).collect::<Vec<String>>();
    units.insert("volume".into(), volume);

    let weight = WeightUnit::list().iter().map(|v| v.fullname().to_string()).collect::<Vec<String>>();
    units.insert("weight".into(), weight);

    let power = PowerUnit::list().iter().map(|v| v.fullname().to_string()).collect::<Vec<String>>();
    units.insert("power".into(), power);

    units
}