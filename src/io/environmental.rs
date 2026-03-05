//! Acc Environmental Information
//!

use acc_interface::datatypes::{
    weather_t, DayNight, DistractionLevelStatusT, DrowsyLevelStatusT, FogClassification,
    OmDriverDistractionLevelFiltered, OmDriverDrowsyLevel, PrecipitationLevel, ValueState,
};
use num_derive::{FromPrimitive, ToPrimitive};

/// Fog Classification
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Fog {
    /// Excellent Visibility
    ExcellentVisibility,
    /// Good Visibility
    GoodVisibility,
    /// Moderate Visibility
    ModerateVisibility,
    /// Poor Visiblity
    PoorVisibility,
    /// Mist
    Mist,
    /// Light
    Light,
    /// Thick
    Thick,
    /// Dense
    Dense,
    /// Unknown
    #[default]
    Unknown,
}

impl From<&FogClassification> for Fog {
    fn from(val: &FogClassification) -> Self {
        use Fog::*;
        use FogClassification::*;
        match val {
            FOG_CLASSIFICATION_UNKNOWN => Unknown,
            FOG_CLASSIFICATION_DENSE => Dense,
            FOG_CLASSIFICATION_EXCELLENT_VISIBILITY => ExcellentVisibility,
            FOG_CLASSIFICATION_GOOD_VISIBILITY => GoodVisibility,
            FOG_CLASSIFICATION_LIGHT => Light,
            FOG_CLASSIFICATION_MIST => Mist,
            FOG_CLASSIFICATION_MODERATE_VISIBILITY => ModerateVisibility,
            FOG_CLASSIFICATION_POOR_VISIBILITY => PoorVisibility,
            FOG_CLASSIFICATION_THICK => Thick,
        }
    }
}
impl From<&Fog> for Option<usize> {
    fn from(field: &Fog) -> Option<usize> {
        use Fog::*;
        match field {
            ExcellentVisibility => Some(0),
            GoodVisibility => Some(1),
            ModerateVisibility => Some(2),
            PoorVisibility => Some(3),
            Mist => Some(4),
            Light => Some(5),
            Thick => Some(6),
            Dense => Some(7),
            Unknown => None,
        }
    }
}
/// Day Night Information
#[derive(Debug, Default, Clone, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum DayPeriod {
    /// Day
    Day,
    /// Night
    Night,
    /// Night
    Dawn,
    /// Dusk
    Dusk,
    #[default]
    /// Unknown
    Unknown,
}

impl From<&DayNight> for DayPeriod {
    fn from(val: &DayNight) -> Self {
        use DayNight::*;
        match val {
            DN_DAWN => DayPeriod::Dawn,
            DN_DAY => DayPeriod::Day,
            DN_DUSK => DayPeriod::Dusk,
            DN_NIGHT => DayPeriod::Night,
            DN_UNKNOWN => DayPeriod::Unknown,
        }
    }
}

impl From<&DayPeriod> for Option<usize> {
    fn from(val: &DayPeriod) -> Option<usize> {
        match val {
            DayPeriod::Day => Some(0),
            DayPeriod::Night => Some(1),
            DayPeriod::Dusk => Some(2),
            DayPeriod::Dawn => Some(3),
            DayPeriod::Unknown => None,
        }
    }
}

/// Rain Levels
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum RainLevels {
    /// Light
    Light,
    /// Moderate
    Moderate,
    /// Heavy
    Heavy,
    #[default]
    /// Unvailable
    Unvailable,
}

impl From<&PrecipitationLevel> for RainLevels {
    fn from(val: &PrecipitationLevel) -> Self {
        use PrecipitationLevel::*;
        match val {
            PL_LIGHT => RainLevels::Light,
            PL_HEAVY => RainLevels::Heavy,
            PL_MODERATE => RainLevels::Moderate,
            PL_NONE => RainLevels::Unvailable,
        }
    }
}

impl From<&RainLevels> for Option<usize> {
    fn from(val: &RainLevels) -> Option<usize> {
        match val {
            RainLevels::Light => Some(0),
            RainLevels::Moderate => Some(1),
            RainLevels::Heavy => Some(2),
            RainLevels::Unvailable => None,
        }
    }
}

/// Distraction Levels
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum DistractionLevels {
    /// Focused
    Focused = 0,
    /// Distracted
    Distracted = 1,
    /// None
    #[default]
    Unvailable,
}

impl From<&OmDriverDistractionLevelFiltered> for DistractionLevels {
    fn from(val: &OmDriverDistractionLevelFiltered) -> Self {
        use DistractionLevelStatusT::*;
        match val.valueState {
            ValueState::VALUE_STATE_VALID => match val.driverDistractionLevelFiltered {
                DLS_DISTRACTED => DistractionLevels::Distracted,
                DLS_FOCUSED => DistractionLevels::Focused,
                DLS_UNAVAILABLE => DistractionLevels::Unvailable,
            },
            ValueState::VALUE_STATE_INVALID | ValueState::VALUE_STATE_UNAVAILABLE => {
                Self::default()
            }
        }
    }
}

impl From<&DistractionLevels> for Option<usize> {
    fn from(val: &DistractionLevels) -> Option<usize> {
        use DistractionLevels::*;
        match val {
            Distracted => Some(0),
            Focused | Unvailable => None,
        }
    }
}

/// Drowsy Levels
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum DrowsyLevels {
    /// Drowsy Level 1
    Level1,
    /// Drowsy Level 2
    Level2,
    /// Drowsy Level 3
    Level3,
    /// Drowsy Level 4
    Level4,
    /// Drowsy Level 5
    Level5,
    /// Drowsy Level 6
    Level6,
    /// Drowsy Level 7
    Level7,
    /// Drowsy Level 8
    Level8,
    /// Drowsy Level 9
    Level9,
    /// Unvailable
    #[default]
    Unavailable,
}

impl From<&OmDriverDrowsyLevel> for DrowsyLevels {
    fn from(val: &OmDriverDrowsyLevel) -> Self {
        use DrowsyLevelStatusT::*;
        use DrowsyLevels::*;
        match val.valueState {
            ValueState::VALUE_STATE_VALID => match val.driverDrowsyLevel {
                DROWSY_LEVEL_STATUS_LEVEL1 => Level1,
                DROWSY_LEVEL_STATUS_LEVEL2 => Level2,
                DROWSY_LEVEL_STATUS_LEVEL3 => Level3,
                DROWSY_LEVEL_STATUS_LEVEL4 => Level4,
                DROWSY_LEVEL_STATUS_LEVEL5 => Level5,
                DROWSY_LEVEL_STATUS_LEVEL6 => Level6,
                DROWSY_LEVEL_STATUS_LEVEL7 => Level7,
                DROWSY_LEVEL_STATUS_LEVEL8 => Level8,
                DROWSY_LEVEL_STATUS_LEVEL9 => Level9,
                DROWSY_LEVEL_STATUS_UNAVAILABLE => Self::Unavailable,
            },
            ValueState::VALUE_STATE_INVALID | ValueState::VALUE_STATE_UNAVAILABLE => {
                Self::default()
            }
        }
    }
}

impl From<&DrowsyLevels> for Option<usize> {
    fn from(value: &DrowsyLevels) -> Option<usize> {
        match value {
            DrowsyLevels::Level1 => Some(0),
            DrowsyLevels::Level2 => Some(1),
            DrowsyLevels::Level3 => Some(2),
            DrowsyLevels::Level4 => Some(3),
            DrowsyLevels::Level5 => Some(4),
            DrowsyLevels::Level6 => Some(5),
            DrowsyLevels::Level7 => Some(6),
            DrowsyLevels::Level8 => Some(7),
            DrowsyLevels::Level9 => Some(8),
            DrowsyLevels::Unavailable => None,
        }
    }
}

/// Weather Informations
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Weather {
    /// Fog
    pub fog: Fog,
    /// Day Night
    pub day_night: DayPeriod,
    /// Rain Levels,
    pub rain: RainLevels,
}

/// Occupant Monitor Informations
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OmInformations {
    /// Distraction Levels,
    pub distraction: DistractionLevels,
    /// Drowsy Levels
    pub drowsy: DrowsyLevels,
}
/// All Interdistance Inputs
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IaccInformations {
    /// Weather Informations
    pub weath: Weather,
    /// Om Informations
    pub om: OmInformations,
}

impl From<&weather_t> for Weather {
    fn from(val: &weather_t) -> Self {
        Weather {
            fog: (&val.fog.level).into(),
            day_night: (&val.daynight).into(),
            // TODO: Maybe Check for confidence
            // Not Mentionned in Sfreq
            rain: (&val.precipitation.level).into(),
        }
    }
}

#[cfg(feature = "caros")]
impl From<&catalog_ampere_adas::sdv_adas_environmentalinformation::Weather_t> for Weather {
    fn from(val: &catalog_ampere_adas::sdv_adas_environmentalinformation::Weather_t) -> Self {
        use num_traits::FromPrimitive;
        let fog = (&val
            .fog
            .as_ref()
            .and_then(|v| FogClassification::from_i32(v.level.value()))
            .unwrap_or_default())
            .into();
        let day_night = (&DayNight::from_i32(val.daynight.value()).unwrap_or_default()).into();
        let rain = (&val
            .precipitation
            .as_ref()
            .and_then(|v| PrecipitationLevel::from_i32(v.level.value()))
            .unwrap_or_default())
            .into();
        Self {
            fog,
            day_night,
            rain,
        }
    }
}

impl IaccInformations {
    /// update Iacc Drowsy Levels
    pub fn update_drowsy(&mut self, drowsy: DrowsyLevels) {
        self.om.drowsy = drowsy;
    }
    /// Update Iacc Distraction Levels
    pub fn update_distraction(&mut self, distraction: DistractionLevels) {
        self.om.distraction = distraction;
    }
    /// Update Iacc Distraction Levels
    pub fn update_weather(&mut self, weather: Weather) {
        self.weath = weather;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use acc_interface::datatypes::{
        fog_t, precipitation_t, weather_t, DayNight, PrecipitationLevel,
    };

    #[test]
    fn test_fog_conversion() {
        // Test all variants of Fog
        let cases = vec![
            (Fog::ExcellentVisibility, Some(0)),
            (Fog::GoodVisibility, Some(1)),
            (Fog::ModerateVisibility, Some(2)),
            (Fog::PoorVisibility, Some(3)),
            (Fog::Mist, Some(4)),
            (Fog::Light, Some(5)),
            (Fog::Thick, Some(6)),
            (Fog::Dense, Some(7)),
            (Fog::Unknown, None),
        ];
        for (input, expected) in cases {
            assert_eq!(Option::<usize>::from(&input), expected);
        }
    }

    #[test]
    fn test_day_period_conversion() {
        // Test all variants of DayPeriod
        let cases = vec![
            (DayPeriod::Day, Some(0)),
            (DayPeriod::Night, Some(1)),
            (DayPeriod::Dusk, Some(2)),
            (DayPeriod::Dawn, Some(3)),
            (DayPeriod::Unknown, None),
        ];
        for (input, expected) in cases {
            assert_eq!(Option::<usize>::from(&input), expected);
        }
    }

    #[test]
    fn test_rain_levels_conversion() {
        // Test all variants of RainLevels
        let cases = vec![
            (RainLevels::Light, Some(0)),
            (RainLevels::Moderate, Some(1)),
            (RainLevels::Heavy, Some(2)),
            (RainLevels::Unvailable, None),
        ];
        for (input, expected) in cases {
            assert_eq!(Option::<usize>::from(&input), expected);
        }
    }

    #[test]
    fn test_distraction_levels_conversion() {
        // Test all variants of DistractionLevels
        let cases = vec![
            (DistractionLevels::Focused, None),
            (DistractionLevels::Distracted, Some(0)),
            (DistractionLevels::Unvailable, None),
        ];
        for (input, expected) in cases {
            assert_eq!(Option::<usize>::from(&input), expected);
        }
    }

    #[test]
    fn test_drowsy_levels_conversion() {
        // Test all variants of DrowsyLevels
        let cases = vec![
            (DrowsyLevels::Level1, Some(0)),
            (DrowsyLevels::Level2, Some(1)),
            (DrowsyLevels::Level3, Some(2)),
            (DrowsyLevels::Level4, Some(3)),
            (DrowsyLevels::Level5, Some(4)),
            (DrowsyLevels::Level6, Some(5)),
            (DrowsyLevels::Level7, Some(6)),
            (DrowsyLevels::Level8, Some(7)),
            (DrowsyLevels::Level9, Some(8)),
            (DrowsyLevels::Unavailable, None),
        ];
        for (input, expected) in cases {
            assert_eq!(Option::<usize>::from(&input), expected);
        }
    }

    #[test]
    fn test_weather_conversion() {
        let weather = weather_t {
            fog: fog_t {
                level: FogClassification::FOG_CLASSIFICATION_LIGHT,
                ..Default::default()
            },
            daynight: DayNight::DN_DAWN,
            precipitation: precipitation_t {
                level: PrecipitationLevel::PL_HEAVY,
                ..Default::default()
            },
            ..Default::default()
        };
        let converted: Weather = (&weather).into();
        assert_eq!(converted.fog, Fog::Light);
        assert_eq!(converted.day_night, DayPeriod::Dawn);
        assert_eq!(converted.rain, RainLevels::Heavy);
    }

    #[test]
    fn test_iacc_informations_update() {
        let mut iacc_info = IaccInformations::default();

        let new_weather = Weather {
            fog: Fog::Dense,
            day_night: DayPeriod::Night,
            rain: RainLevels::Moderate,
        };
        iacc_info.update_weather(new_weather.clone());
        assert_eq!(iacc_info.weath, new_weather);

        let new_drowsy = DrowsyLevels::Level5;
        iacc_info.update_drowsy(new_drowsy.clone());
        assert_eq!(iacc_info.om.drowsy, new_drowsy);

        let new_distraction = DistractionLevels::Distracted;
        iacc_info.update_distraction(new_distraction.clone());
        assert_eq!(iacc_info.om.distraction, new_distraction);
    }
}
