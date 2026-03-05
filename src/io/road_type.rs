//! Road type
//!
use core::ops::Div;
#[allow(unused_imports)]
use num_traits::cast::FromPrimitive;

use acc_interface::datatypes::{isaOut_t, RoadType, SpeedUnit};
#[cfg(feature = "caros")]
use catalog_ampere_adas as oem;

use super::{kph, mps, Velocity};

/// Different Type of Road
#[derive(Debug, Default, PartialEq)]
pub enum RoadClass {
    #[default]
    /// off road
    OffRoad,
    /// pedestrian road
    Pedestrian,
    /// bicycle road
    Bicycle,
    /// city road
    City,
    /// rural road
    Rural,
    /// interurban road
    Interurban,
    /// motorway
    Motorway,
}

impl RoadClass {
    /// check if current road is a high-speed road
    pub fn is_high_speed(&self) -> bool {
        matches!(self, RoadClass::Motorway)
    }
}

/// Isa Info
/// This struct contains the information about the road type
/// and the speed limit of the road
#[derive(Debug, Default, PartialEq)]
pub struct IsaInfo {
    /// Osp Speed Limit
    pub osp_speed_limit: Option<Velocity>,
    /// road class
    pub road_class: RoadClass,
}

impl From<RoadType> for RoadClass {
    fn from(value: RoadType) -> Self {
        match value {
            RoadType::RT_OFFROAD => Self::OffRoad,
            RoadType::RT_PEDESTRIAN => Self::Pedestrian,
            RoadType::RT_BICYCLE => Self::Bicycle,
            RoadType::RT_CITY => Self::City,
            RoadType::RT_RURAL => Self::Rural,
            RoadType::RT_INTERURBAN => Self::Interurban,
            RoadType::RT_MOTORWAY => Self::Motorway,
        }
    }
}
#[cfg(feature = "caros")]
impl From<oem::sdv_adas_adas_common_types::RoadType> for RoadClass {
    fn from(value: oem::sdv_adas_adas_common_types::RoadType) -> Self {
        use oem::sdv_adas_adas_common_types::RoadType::*;
        match value {
            RT_OFFROAD => Self::OffRoad,
            RT_PEDESTRIAN => Self::Pedestrian,
            RT_BICYCLE => Self::Bicycle,
            RT_CITY => Self::City,
            RT_RURAL => Self::Rural,
            RT_INTERURBAN => Self::Interurban,
            RT_MOTORWAY => Self::Motorway,
        }
    }
}

impl From<&isaOut_t> for IsaInfo {
    fn from(value: &isaOut_t) -> Self {
        Self {
            osp_speed_limit: match value.speed_unit {
                SpeedUnit::SPEED_UNIT_UNKNOWN => None,
                SpeedUnit::SPEED_UNIT_KILOMETER_PER_HOUR => {
                    Some(Velocity::new::<kph>(value.main_speed.speed.into()))
                }
                SpeedUnit::SPEED_UNIT_MILES_PER_HOUR => Some(Velocity::new::<mps>(
                    f32::from(value.main_speed.speed).div(2.237),
                )),
            },
            road_class: value.isa_info.road_type.into(),
        }
    }
}

#[cfg(feature = "caros")]
impl From<&oem::sdv_adas_intelligent_speed_assist_types::IsaOut_t> for IsaInfo {
    fn from(value: &oem::sdv_adas_intelligent_speed_assist_types::IsaOut_t) -> Self {
        use oem::sdv_common_types::SpeedUnit;
        Self {
            osp_speed_limit: match value.speed_unit.enum_value_or_default() {
                SpeedUnit::SPEED_UNIT_UNKNOWN => None,
                SpeedUnit::SPEED_UNIT_KILOMETER_PER_HOUR => Some(Velocity::new::<kph>(
                    f32::from_u32(value.main_speed.speed).unwrap_or_default(),
                )),
                SpeedUnit::SPEED_UNIT_MILES_PER_HOUR => Some(Velocity::new::<mps>(
                    f32::from_u32(value.main_speed.speed)
                        .unwrap_or_default()
                        .div(2.237),
                )),
            },
            road_class: value.isa_info.road_type.enum_value_or_default().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{RoadClass, RoadType};

    #[test]
    fn test_type_conversion() {
        let mut road_class: RoadClass = RoadType::RT_OFFROAD.into();
        assert_eq!(road_class, RoadClass::OffRoad);
        road_class = RoadType::RT_PEDESTRIAN.into();
        assert_eq!(road_class, RoadClass::Pedestrian);
        road_class = RoadType::RT_BICYCLE.into();
        assert_eq!(road_class, RoadClass::Bicycle);
        road_class = RoadType::RT_CITY.into();
        assert_eq!(road_class, RoadClass::City);
        road_class = RoadType::RT_RURAL.into();
        assert_eq!(road_class, RoadClass::Rural);
        road_class = RoadType::RT_INTERURBAN.into();
        assert_eq!(road_class, RoadClass::Interurban);
        road_class = RoadType::RT_MOTORWAY.into();
        assert_eq!(road_class, RoadClass::Motorway);
    }
}
