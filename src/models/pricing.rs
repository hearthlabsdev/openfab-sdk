use crate::units::*;
use crate::utils::*;
#[cfg(feature = "schema")]
use rocket_okapi::okapi::{schemars, schemars::JsonSchema};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uom::si::f64::*;
use uuid::Uuid;

#[cfg(feature = "fromform")]
use rocket::{FromForm, FromFormField};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseReconciliationError(pub String);

impl fmt::Display for ParseReconciliationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid reconciliation strategy: '{}'. Expected one of: Additive, XorHigh, XorLow, XorMid", self.0)
    }
}

impl std::error::Error for ParseReconciliationError {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "fromform", derive(FromFormField))]
/// how to reconcile divergent pricing models
pub enum Reconciliation {
    /// used to indicate cost estimate is base + time + material
    Additive,
    /// used to select whichever value is highest price
    XorHigh,
    /// Lowest Price
    XorLow,
    /// the median value between price estimates
    XorMid,
}

impl fmt::Display for Reconciliation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Reconciliation::Additive => "Additive",
            Reconciliation::XorHigh => "XorHigh",
            Reconciliation::XorLow => "XorLow",
            Reconciliation::XorMid => "XorMid",
        };
        write!(f, "{}", name)
    }
}

impl FromStr for Reconciliation {
    type Err = ParseReconciliationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_lowercase();
        match normalized.as_str() {
            "additive" => Ok(Reconciliation::Additive),
            "xorhigh" | "xor_high" | "high" => Ok(Reconciliation::XorHigh),
            "xorlow" | "xor_low" | "low" => Ok(Reconciliation::XorLow),
            "xormid" | "xor_mid" | "mid" | "median" => Ok(Reconciliation::XorMid),
            _ => Err(ParseReconciliationError(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PricingProfile {
    pub uid: Uuid,
    pub name: String,

    /// base charge per print.
    pub base: f64,

    /// price per base unit time.
    per_sec: Option<f64>,
    time_unit: Option<TimeUnit>,

    per_meter: Option<f64>,
    length_unit: Option<LengthUnit>,

    per_sqm: Option<f64>,
    area_unit: Option<AreaUnit>,

    per_watt: Option<f64>,
    power_unit: Option<PowerUnit>,

    per_liter: Option<f64>,
    volume_unit: Option<VolumeUnit>,

    per_kg: Option<f64>,
    weight_unit: Option<WeightUnit>,

    /// price per count of something
    pub per_count: Option<f64>,
    pub count_name: Option<String>,

    pub created_at: i64,
    /// intra and inter profile reconiliation strategy, uses the most specific reconciliation in the hierarchy.
    pub reconciliation: Reconciliation,
}

impl PricingProfile {

    pub fn new(
        uid: Uuid,
        name: impl Into<String>,
        base: f64,
        reconciliation: Reconciliation,
        created_at: i64,
    ) -> Self {
        Self {
            uid,
            name: name.into(),
            base,
            per_sec: None,
            time_unit: None,
            per_meter: None,
            length_unit: None,
            per_sqm: None,
            area_unit: None,
            per_watt: None,
            power_unit: None,
            per_liter: None,
            volume_unit: None,
            per_kg: None,
            weight_unit: None,
            per_count: None,
            count_name: None,
            created_at,
            reconciliation,
        }
    }

    pub fn get_time_unit(&self) -> Option<TimeUnit> {
        self.time_unit
    }

    pub fn get_count_name(&self) -> Option<String> {
        self.count_name.clone()
    }

    pub fn get_length_unit(&self) -> Option<LengthUnit> {
        self.length_unit
    }

    pub fn get_area_unit(&self) -> Option<AreaUnit> {
        self.area_unit
    }

    pub fn get_power_unit(&self) -> Option<PowerUnit> {
        self.power_unit
    }

    pub fn get_volume_unit(&self) -> Option<VolumeUnit> {
        self.volume_unit
    }

    pub fn get_weight_unit(&self) -> Option<WeightUnit> {
        self.weight_unit
    }
    // ==========================================
    // TIME (Canonical Base: Seconds)
    // ==========================================
    pub fn set_time(&mut self, rate: f64, unit: TimeUnit) {
        let canonical = match unit {
            TimeUnit::Second => {
                Time::new::<uom::si::time::second>(1.0).get::<uom::si::time::second>()
            }
            TimeUnit::Minute => {
                Time::new::<uom::si::time::minute>(1.0).get::<uom::si::time::second>()
            }
            TimeUnit::Hour => Time::new::<uom::si::time::hour>(1.0).get::<uom::si::time::second>(),
        };
        self.per_sec = Some(rate / canonical);
        self.time_unit = Some(unit);
    }

    pub fn get_time_as(&self, unit: TimeUnit) -> Option<f64> {
        let base_rate = self.per_sec?;
        let canonical = Time::new::<uom::si::time::second>(1.0);
        Some(match unit {
            TimeUnit::Second => base_rate / canonical.get::<uom::si::time::second>(),
            TimeUnit::Minute => base_rate / canonical.get::<uom::si::time::minute>(),
            TimeUnit::Hour => base_rate / canonical.get::<uom::si::time::hour>(),
        })
    }

    pub fn get_time(&self) -> Option<f64> {
        let unit = self.time_unit?;
        self.get_time_as(unit)
    }

    pub fn get_volume_as(&self, unit: VolumeUnit) -> Option<f64> {
        let base_rate = self.per_liter?;
        let converted = Volume::new::<uom::si::volume::liter>(base_rate);
        Some(match unit {
            VolumeUnit::Milliliter => converted.get::<uom::si::volume::milliliter>(),
            VolumeUnit::Liter => converted.get::<uom::si::volume::liter>(),
        })
    }

    pub fn get_volume(&self) -> Option<f64> {
        let unit = self.volume_unit?;
        self.get_volume_as(unit)
    }
    // ==========================================
    // LENGTH (Canonical Base: Meters)
    // ==========================================

    // ==========================================
    // VOLUME (Canonical Base: Liters)
    // ==========================================
    pub fn set_volume(&mut self, rate: f64, unit: VolumeUnit) {
        let canonical = match unit {
            VolumeUnit::Milliliter => {
                Volume::new::<uom::si::volume::milliliter>(1.0).get::<uom::si::volume::liter>()
            }
            VolumeUnit::Liter => {
                Volume::new::<uom::si::volume::liter>(1.0).get::<uom::si::volume::liter>()
            }
        };
        self.per_liter = Some(rate / canonical);
        self.volume_unit = Some(unit);
    }
    // meter is the base unit so 50 / centimeter means 5,000 per meter
    pub fn set_length(&mut self, rate: f64, unit: LengthUnit) {
        let canonical = match unit {
            LengthUnit::Millimeter => {
                Length::new::<uom::si::length::millimeter>(1.0).get::<uom::si::length::meter>()
            }
            LengthUnit::Centimeter => {
                Length::new::<uom::si::length::centimeter>(1.0).get::<uom::si::length::meter>()
            }
            LengthUnit::Meter => {
                Length::new::<uom::si::length::meter>(1.0).get::<uom::si::length::meter>()
            }
            LengthUnit::Kilometer => {
                Length::new::<uom::si::length::kilometer>(1.0).get::<uom::si::length::meter>()
            }
        };
        self.per_meter = Some(rate / canonical);
        self.length_unit = Some(unit);
    }
    pub fn get_length_as(&self, unit: LengthUnit) -> Option<f64> {
        let base_rate = self.per_meter?;
        let canonical = Length::new::<uom::si::length::meter>(1.0);
        Some(match unit {
            LengthUnit::Millimeter => base_rate / canonical.get::<uom::si::length::millimeter>(),
            LengthUnit::Centimeter => base_rate / canonical.get::<uom::si::length::centimeter>(),
            LengthUnit::Meter => base_rate / canonical.get::<uom::si::length::meter>(),
            LengthUnit::Kilometer => base_rate / canonical.get::<uom::si::length::kilometer>(),
        })
    }

    pub fn get_length(&self) -> Option<f64> {
        let unit = self.length_unit?;
        self.get_length_as(unit)
    }

    // ==========================================
    // WEIGHT (Canonical Base: Kilograms)
    // ==========================================

    pub fn set_weight(&mut self, rate: f64, unit: WeightUnit) {
        let canonical = match unit {
            WeightUnit::Milligram => {
                Mass::new::<uom::si::mass::milligram>(1.0).get::<uom::si::mass::kilogram>()
            }
            WeightUnit::Gram => {
                Mass::new::<uom::si::mass::gram>(1.0).get::<uom::si::mass::kilogram>()
            }
            WeightUnit::Kilogram => {
                Mass::new::<uom::si::mass::kilogram>(1.0).get::<uom::si::mass::kilogram>()
            }
        };
        self.per_kg = Some(rate / canonical);
        self.weight_unit = Some(unit);
    }

    pub fn get_weight_as(&self, unit: WeightUnit) -> Option<f64> {
        let base_rate = self.per_kg?;
        let canonical = Mass::new::<uom::si::mass::kilogram>(1.0);
        Some(match unit {
            WeightUnit::Milligram => base_rate / canonical.get::<uom::si::mass::milligram>(),
            WeightUnit::Gram => base_rate / canonical.get::<uom::si::mass::gram>(),
            WeightUnit::Kilogram => base_rate / canonical.get::<uom::si::mass::kilogram>(),
        })
    }

    pub fn get_weight(&self) -> Option<f64> {
        let unit = self.weight_unit?;
        self.get_weight_as(unit)
    }

    pub fn set_area(&mut self, rate: f64, unit: AreaUnit) {
        let canonical = match unit {
            AreaUnit::SquareMillimeter => Area::new::<uom::si::area::square_millimeter>(1.0)
                .get::<uom::si::area::square_meter>(),
            AreaUnit::SquareCentimeter => Area::new::<uom::si::area::square_centimeter>(1.0)
                .get::<uom::si::area::square_meter>(),
            AreaUnit::SquareMeter => {
                Area::new::<uom::si::area::square_meter>(1.0).get::<uom::si::area::square_meter>()
            }
        };
        self.per_sqm = Some(rate / canonical);
        self.area_unit = Some(unit);
    }

    pub fn get_area_as(&self, unit: AreaUnit) -> Option<f64> {
        let base_rate = self.per_sqm?;
        let canonical = Area::new::<uom::si::area::square_meter>(1.0);
        Some(match unit {
            AreaUnit::SquareMillimeter => base_rate / canonical.get::<uom::si::area::square_millimeter>(),
            AreaUnit::SquareCentimeter => base_rate / canonical.get::<uom::si::area::square_centimeter>(),
            AreaUnit::SquareMeter => base_rate / canonical.get::<uom::si::area::square_meter>(),
        })
    }

    pub fn get_area(&self) -> Option<f64> {
        let unit = self.area_unit?;
        self.get_area_as(unit)
    }

    pub fn set_power(&mut self, rate: f64, unit: PowerUnit) {
        let canonical = match unit {
            PowerUnit::Milliwatt => {
                Power::new::<uom::si::power::milliwatt>(1.0).get::<uom::si::power::watt>()
            }
            PowerUnit::Watt => {
                Power::new::<uom::si::power::watt>(1.0).get::<uom::si::power::watt>()
            }
            PowerUnit::Kilowatt => {
                Power::new::<uom::si::power::kilowatt>(1.0).get::<uom::si::power::watt>()
            }
            PowerUnit::Megawatt => {
                Power::new::<uom::si::power::megawatt>(1.0).get::<uom::si::power::watt>()
            }
        };
        self.per_watt = Some(rate / canonical);
        self.power_unit = Some(unit);
    }

    pub fn get_power_as(&self, unit: PowerUnit) -> Option<f64> {
        let base_rate = self.per_watt?;
        let canonical = Power::new::<uom::si::power::watt>(1.0);
        Some(match unit {
            PowerUnit::Milliwatt => base_rate / canonical.get::<uom::si::power::milliwatt>(),
            PowerUnit::Watt => base_rate / canonical.get::<uom::si::power::watt>(),
            PowerUnit::Kilowatt => base_rate / canonical.get::<uom::si::power::kilowatt>(),
            PowerUnit::Megawatt => base_rate / canonical.get::<uom::si::power::megawatt>(),
        })
    }

    pub fn get_power(&self) -> Option<f64> {
        let unit = self.power_unit?;
        self.get_power_as(unit)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "fromform", derive(FromForm))]
pub struct CreatePricingProfile {
    pub name: String,
    is_normalized: Option<bool>,
    /// base charge per print.
    pub base: f64,

    /// price per base unit time.
    per_sec: Option<f64>,
    time_unit: Option<TimeUnit>,

    per_meter: Option<f64>,
    length_unit: Option<LengthUnit>,

    per_sqm: Option<f64>,
    area_unit: Option<AreaUnit>,

    per_liter: Option<f64>,
    volume_unit: Option<VolumeUnit>,

    per_kg: Option<f64>,
    weight_unit: Option<WeightUnit>,

    /// price per canonical base unit power (Watts)
    per_watt: Option<f64>,
    power_unit: Option<PowerUnit>,

    /// price per count of something
    /// same to expose as public bc no unit conversions
    pub per_count: Option<f64>,
    pub count_name: Option<String>,

    /// intra and inter profile reconiliation strategy, uses the most specific reconciliation in the hierarchy.
    pub reconciliation: Reconciliation,
}

impl CreatePricingProfile {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_normalized: Some(false),
            base: 0.0,
            per_sec: None,
            time_unit: None,
            per_meter: None,
            length_unit: None,
            per_sqm: None,
            area_unit: None,
            per_liter: None,
            volume_unit: None,
            per_kg: None,
            weight_unit: None,
            per_count: None,
            count_name: None,
            per_watt: None,
            power_unit: None,
            reconciliation: Reconciliation::Additive, // sensible default
        }
    }

    pub fn base(mut self, val: f64) -> Self {
        self.base = val;
        self
    }

    pub fn reconciliation(mut self, rec: Reconciliation) -> Self {
        self.reconciliation = rec;
        self
    }

    // ==========================================
    // Simple Builder Setters (No Normalization)
    // ==========================================

    pub fn time(mut self, rate: f64, unit: TimeUnit) -> Self {
        self.per_sec = Some(rate);
        self.time_unit = Some(unit);
        self
    }

    pub fn length(mut self, rate: f64, unit: LengthUnit) -> Self {
        self.per_meter = Some(rate);
        self.length_unit = Some(unit);
        self
    }

    pub fn area(mut self, rate: f64, unit: AreaUnit) -> Self {
        self.per_sqm = Some(rate);
        self.area_unit = Some(unit);
        self
    }

    pub fn volume(mut self, rate: f64, unit: VolumeUnit) -> Self {
        self.per_liter = Some(rate);
        self.volume_unit = Some(unit);
        self
    }

    pub fn weight(mut self, rate: f64, unit: WeightUnit) -> Self {
        self.per_kg = Some(rate);
        self.weight_unit = Some(unit);
        self
    }

    pub fn power(mut self, rate: f64, unit: PowerUnit) -> Self {
        self.per_watt = Some(rate);
        self.power_unit = Some(unit);
        self
    }

    // ==========================================
    // Private Normalization Helpers
    // ==========================================

    fn normalize_time(mut self, rate: f64, unit: TimeUnit) -> Self {
        let canonical = match unit {
            TimeUnit::Second => {
                Time::new::<uom::si::time::second>(1.0).get::<uom::si::time::second>()
            }
            TimeUnit::Minute => {
                Time::new::<uom::si::time::minute>(1.0).get::<uom::si::time::second>()
            }
            TimeUnit::Hour => Time::new::<uom::si::time::hour>(1.0).get::<uom::si::time::second>(),
        };
        println!("dividing time: {} by {}", rate, canonical);
        self.per_sec = Some(rate / canonical);
        self.time_unit = Some(unit);
        self
    }

    fn normalize_length(mut self, rate: f64, unit: LengthUnit) -> Self {
        let canonical = match unit {
            LengthUnit::Millimeter => {
                Length::new::<uom::si::length::millimeter>(1.0).get::<uom::si::length::meter>()
            }
            LengthUnit::Centimeter => {
                Length::new::<uom::si::length::centimeter>(1.0).get::<uom::si::length::meter>()
            }
            LengthUnit::Meter => {
                Length::new::<uom::si::length::meter>(1.0).get::<uom::si::length::meter>()
            }
            LengthUnit::Kilometer => {
                Length::new::<uom::si::length::kilometer>(1.0).get::<uom::si::length::meter>()
            }
        };
        println!("dividing length: {} by {}", rate, canonical);
        self.per_meter = Some(rate / canonical);
        self.length_unit = Some(unit);
        println!("per meter: {:?}", self.per_meter);
        self
    }

    fn normalize_area(mut self, rate: f64, unit: AreaUnit) -> Self {
        let canonical = match unit {
            AreaUnit::SquareMillimeter => Area::new::<uom::si::area::square_millimeter>(1.0)
                .get::<uom::si::area::square_meter>(),
            AreaUnit::SquareCentimeter => Area::new::<uom::si::area::square_centimeter>(1.0)
                .get::<uom::si::area::square_meter>(),
            AreaUnit::SquareMeter => {
                Area::new::<uom::si::area::square_meter>(1.0).get::<uom::si::area::square_meter>()
            }
        };
        println!("dividing area: {} by {}", rate, canonical);
        self.per_sqm = Some(rate / canonical);
        self.area_unit = Some(unit);
        self
    }

    fn normalize_volume(mut self, rate: f64, unit: VolumeUnit) -> Self {
        let canonical = match unit {
            VolumeUnit::Milliliter => {
                Volume::new::<uom::si::volume::milliliter>(1.0).get::<uom::si::volume::liter>()
            }
            VolumeUnit::Liter => {
                Volume::new::<uom::si::volume::liter>(1.0).get::<uom::si::volume::liter>()
            }
        };
        println!("dividing volume: {} by {}", rate, canonical);
        self.per_liter = Some(rate / canonical);
        self.volume_unit = Some(unit);
        self
    }

    fn normalize_weight(mut self, rate: f64, unit: WeightUnit) -> Self {
        let canonical = match unit {
            WeightUnit::Milligram => {
                Mass::new::<uom::si::mass::milligram>(1.0).get::<uom::si::mass::kilogram>()
            }
            WeightUnit::Gram => {
                Mass::new::<uom::si::mass::gram>(1.0).get::<uom::si::mass::kilogram>()
            }
            WeightUnit::Kilogram => {
                Mass::new::<uom::si::mass::kilogram>(1.0).get::<uom::si::mass::kilogram>()
            }
        };
        println!("dividing weight: {} by {}", rate, canonical);
        self.per_kg = Some(rate / canonical);
        self.weight_unit = Some(unit);
        self
    }

    fn normalize_power(mut self, rate: f64, unit: PowerUnit) -> Self {
        let canonical = match unit {
            PowerUnit::Milliwatt => {
                Power::new::<uom::si::power::milliwatt>(1.0).get::<uom::si::power::watt>()
            }
            PowerUnit::Watt => {
                Power::new::<uom::si::power::watt>(1.0).get::<uom::si::power::watt>()
            }
            PowerUnit::Kilowatt => {
                Power::new::<uom::si::power::kilowatt>(1.0).get::<uom::si::power::watt>()
            }
            PowerUnit::Megawatt => {
                Power::new::<uom::si::power::megawatt>(1.0).get::<uom::si::power::watt>()
            }
        };
        println!("multiplying power: {} by {}", rate, canonical);
        self.per_watt = Some(rate * canonical);
        self.power_unit = Some(unit);
        self
    }

    /// Converts all populated rates and unit strings into canonical base units
    /// using type information and uom scaling.
    pub fn normalize(mut self) -> Self {
        // already normalized return
        if self.is_normalized == Some(true) {
            return self;
        }
        // ==========================================
        // 1. WEIGHT (Canonical Base: Kilograms)
        // ==========================================
        match (self.per_kg, self.weight_unit) {
            (Some(val), Some(unit)) => {
                self = self.normalize_weight(val, unit);
            }
            (Some(val), None) => {
                self = self.normalize_weight(val, WeightUnit::default());
            }
            (None, Some(unit)) => {
                self = self.normalize_weight(0.0, unit);
            }
            (None, None) => {}
        }

        // ==========================================
        // 2. VOLUME (Canonical Base: Liters)
        // ==========================================
        match (self.per_liter, self.volume_unit) {
            (Some(val), Some(unit)) => {
                self = self.normalize_volume(val, unit);
            }
            (Some(val), None) => {
                self = self.normalize_volume(val, VolumeUnit::default());
            }
            (None, Some(unit)) => {
                self = self.normalize_volume(0.0, unit);
            }
            (None, None) => {}
        }

        // ==========================================
        // 3. TIME (Canonical Base: Seconds)
        // ==========================================
        match (self.per_sec, self.time_unit) {
            (Some(val), Some(unit)) => {
                self = self.normalize_time(val, unit);
            }
            (Some(val), None) => {
                self = self.normalize_time(val, TimeUnit::Second);
            }
            (None, Some(unit)) => {
                self = self.normalize_time(0.0, unit);
            }
            (None, None) => {}
        }

        // ==========================================
        // 4. LENGTH (Canonical Base: Meters)
        // ==========================================
        match (self.per_meter, self.length_unit) {
            (Some(val), Some(unit)) => {
                self = self.normalize_length(val, unit);
            }
            (Some(val), None) => {
                self = self.normalize_length(val, LengthUnit::default());
            }
            (None, Some(unit)) => {
                self = self.normalize_length(0.0, unit);
            }
            (None, None) => {}
        }

        // ==========================================
        // 5. AREA (Canonical Base: Square Meters)
        // ==========================================
        match (self.per_sqm, self.area_unit) {
            (Some(val), Some(unit)) => {
                self = self.normalize_area(val, unit);
            }
            (Some(val), None) => {
                self = self.normalize_area(val, AreaUnit::default());
            }
            (None, Some(unit)) => {
                self = self.normalize_area(0.0, unit);
            }
            (None, None) => {}
        }

        // ==========================================
        // 6. POWER (Canonical Base: Watts)
        // ==========================================
        match (self.per_watt, self.power_unit) {
            (Some(val), Some(unit)) => {
                self = self.normalize_power(val, unit);
            }
            (Some(val), None) => {
                self = self.normalize_power(val, PowerUnit::default());
            }
            (None, Some(unit)) => {
                self = self.normalize_power(0.0, unit);
            }
            (None, None) => {}
        }

        self
    }

    /// this is for testing and server usage
    fn into_pricing_profile(self) -> PricingProfile {
        PricingProfile {
            uid: Uuid::new_v4(),
            name: self.name,

            /// base charge per print.
            base: self.base,

            /// price per base unit time.
            per_sec: self.per_sec,
            time_unit: self.time_unit,

            per_meter: self.per_meter,
            length_unit: self.length_unit,

            per_sqm: self.per_sqm,
            area_unit: self.area_unit,

            per_watt: self.per_watt,
            power_unit: self.power_unit,

            per_liter: self.per_liter,
            volume_unit: self.volume_unit,

            per_kg: self.per_kg,
            weight_unit: self.weight_unit,

            /// price per count of something
            per_count: self.per_count,
            count_name: self.count_name,

            created_at: unix_epoch_seconds(),
            /// intra and inter profile reconiliation strategy, uses the most specific reconciliation in the hierarchy.
            reconciliation: self.reconciliation,
        }
    }
}

impl From<CreatePricingProfile> for PricingProfile {
    fn from(profile: CreatePricingProfile) -> PricingProfile {
        profile.normalize().into_pricing_profile()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pricing_profile_builder_all_methods() {
        // Test building a profile using every single builder method
        let create_profile = CreatePricingProfile::new("Standard Industrial Laser")
            .base(10.0)
            .reconciliation(Reconciliation::XorHigh)
            .time(60.0, TimeUnit::Minute) // 60 per minute -> 1.0 per second canonical
            .length(100.0, LengthUnit::Centimeter) // 100 per cm -> 10000 per meter canonical
            .area(50.0, AreaUnit::SquareCentimeter) // 50 per cm^2 -> 500000 per sqm canonical
            .volume(1000.0, VolumeUnit::Milliliter) // 1000 per mL -> 1_000_000 per liter canonical
            .weight(500.0, WeightUnit::Gram) // 500 per gram -> 500k per kg canonical
            .power(2.5, PowerUnit::Kilowatt); // 2.5 kW -> 2500 Watts canonical

        let profile: PricingProfile = create_profile.into();
        println!("profile: {:?}", profile);
        assert_eq!(profile.name, "Standard Industrial Laser");
        assert_eq!(profile.base, 10.0);
        assert_eq!(profile.reconciliation, Reconciliation::XorHigh);

        // Verify canonical values stored after normalization
        assert!((profile.per_sec.unwrap() - 1.0).abs() < 1e-6);
        assert!((profile.per_meter.unwrap() - 10000.0).abs() < 1e-6);
        assert!((profile.per_sqm.unwrap() - 500000.0).abs() < 1e-6);
        assert!((profile.per_liter.unwrap() - 1_000_000.0).abs() < 1e-6);
        assert!((profile.per_kg.unwrap() - 500000.0).abs() < 1e-6);
        assert!((profile.per_watt.unwrap() - 2500.0).abs() < 1e-6);
    }

    #[test]
    fn test_getters_and_get_as_methods() {
        // Initialize a mock profile to test setters and getters directly
        let mut profile = PricingProfile {
            uid: Uuid::new_v4(),
            name: "Unit Conversion Test".into(),
            base: 0.0,
            per_sec: None,
            time_unit: None,
            per_meter: None,
            length_unit: None,
            per_sqm: None,
            area_unit: None,
            per_watt: None,
            power_unit: None,
            per_liter: None,
            volume_unit: None,
            per_kg: None,
            weight_unit: None,
            per_count: None,
            count_name: None,
            created_at: 0,
            reconciliation: Reconciliation::Additive,
        };

        // --- TIME ---
        profile.set_time(120.0, TimeUnit::Minute);
        println!("time: {:?}", profile.per_sec);
        assert_eq!(profile.time_unit, Some(TimeUnit::Minute));
        // get() returns rate in the stored unit (Minute)
        assert!((profile.get_time().unwrap() - 120.0).abs() < 1e-6);
        // get_as() queries alternative units
        assert!((profile.get_time_as(TimeUnit::Second).unwrap() - 2.0).abs() < 1e-6);
        assert!((profile.get_time_as(TimeUnit::Hour).unwrap() - (120.0 * 60.0)).abs() < 1e-6);

        // --- LENGTH ---
        profile.set_length(50.0, LengthUnit::Centimeter);
        println!("length: {:?}", profile.per_meter);
        assert_eq!(profile.length_unit, Some(LengthUnit::Centimeter));
        assert!((profile.get_length().unwrap() - 50.0).abs() < 1e-6);
        assert!((profile.get_length_as(LengthUnit::Meter).unwrap() - 5_000.0).abs() < 1e-6);
        assert!((profile.get_length_as(LengthUnit::Millimeter).unwrap() - 5.0).abs() < 1e-6);

        // --- WEIGHT ---
        // $500 / gram
        profile.set_weight(500.0, WeightUnit::Gram);
        assert_eq!(profile.weight_unit, Some(WeightUnit::Gram));
        assert!((profile.get_weight().unwrap() - 500.0).abs() < 1e-6);
        assert!((profile.get_weight_as(WeightUnit::Kilogram).unwrap() - 500.0 * 1_000.0).abs() < 1e-6);
        assert!((profile.get_weight_as(WeightUnit::Milligram).unwrap() - 0.5).abs() < 1e-3);

        // --- AREA ---
        profile.set_area(1.0, AreaUnit::SquareCentimeter);
        println!("area: {:?}", profile.per_sqm);
        assert_eq!(profile.area_unit, Some(AreaUnit::SquareCentimeter));
        assert!((profile.get_area().unwrap() - 1.0).abs() < 1e-6);
        assert!((profile.get_area_as(AreaUnit::SquareMeter).unwrap() - (10_000.0)).abs() < 1e-6);

        // --- POWER ---
        profile.set_power(3.5, PowerUnit::Kilowatt);
        assert_eq!(profile.power_unit, Some(PowerUnit::Kilowatt));
        assert!((profile.get_power().unwrap() - 3.5).abs() < 1e-6);
        assert!((profile.get_power_as(PowerUnit::Watt).unwrap() - 0.0035).abs() < 1e-6);
        assert!((profile.get_power_as(PowerUnit::Milliwatt).unwrap() - 0.00_00035).abs() < 1e-3);
    }
}
