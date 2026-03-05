//! Define Vehicule Status Status for ACC
//!
use acc_interface::datatypes::{
    vehicleStatus_t, FrontWiperMode, ModeMexEgo, ProbableRainStatusEgo, RollerBenchDetection,
    VehPwrMode,
};
#[cfg(feature = "caros")]
use catalog_ampere_adas as oem;

/// Mex mode
#[derive(Default, Debug, Clone, PartialEq)]

pub enum MexMode {
    /// Undefined mex mode
    #[default]
    Undefined,
    /// ECO mex mode
    Eco,
    /// Sport mex mode
    Sport,
    /// Comfort mex mode
    Comfort,
}

impl From<ModeMexEgo> for MexMode {
    fn from(value: ModeMexEgo) -> Self {
        match value {
            ModeMexEgo::MME_UNDEFINED => Self::Undefined,
            ModeMexEgo::MME_ECO => Self::Eco,
            ModeMexEgo::MME_COMFORT => Self::Comfort,
            ModeMexEgo::MME_SPORT => Self::Sport,
        }
    }
}

/// Vehicule Status
#[derive(Debug, Default, Clone)]
pub struct VehStatus {
    /// Trailer Presence
    pub trailer_presence: bool,
    /// Front Wiper Status
    pub front_wiper_status: FrWiperMd,
    /// Vehicule Pwr Mode
    pub pwr_mode: PowerMode,
    /// Dynamique MAss
    pub dyn_mass: u16,
    /// Rain Status
    pub rain_status: RainStatus,
    /// Mex Mode
    pub mex_mode: MexMode,
    /// Oem bench mode
    pub oem_bench_mode: bool,
    /// Bench mode switch : true if it changes
    pub bench_mode_switch: bool,
}

impl From<&vehicleStatus_t> for VehStatus {
    fn from(value: &vehicleStatus_t) -> Self {
        Self {
            trailer_presence: value.trailerPresence,
            front_wiper_status: value.frontWiperStatus.into(),
            pwr_mode: value.vehicleStates.into(),
            dyn_mass: value.dynamicMass,
            rain_status: value.probableRainSts.into(),
            mex_mode: value.modeMex.into(),
            oem_bench_mode: value.rollerBenchDetection
                == RollerBenchDetection::ROLLER_BENCH_DETECTION_REQUEST,
            ..Default::default()
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_vehicle_status::ModeMexEgo> for MexMode {
    fn from(value: oem::sdv_adas_ego_ego_vehicle_status::ModeMexEgo) -> Self {
        use oem::sdv_adas_ego_ego_vehicle_status::ModeMexEgo;
        match value {
            ModeMexEgo::MME_UNDEFINED => Self::Undefined,
            ModeMexEgo::MME_ECO => Self::Eco,
            ModeMexEgo::MME_COMFORT => Self::Comfort,
            ModeMexEgo::MME_SPORT => Self::Sport,
        }
    }
}
#[cfg(feature = "caros")]
impl From<&oem::sdv_adas_ego_ego_vehicle_status::VehicleStatus_t> for VehStatus {
    fn from(value: &oem::sdv_adas_ego_ego_vehicle_status::VehicleStatus_t) -> Self {
        use oem::sdv_adas_ego_ego_vehicle_status::RollerBenchDetection;
        Self {
            trailer_presence: value.trailer_presence,
            front_wiper_status: value
                .front_wiper_status
                .enum_value()
                .unwrap_or_default()
                .into(),
            pwr_mode: value.vehicle_states.enum_value().unwrap_or_default().into(),
            dyn_mass: value.dynamic_mass as u16,
            rain_status: value
                .probable_rain_sts
                .enum_value()
                .unwrap_or_default()
                .into(),
            mex_mode: value.mode_mex.enum_value_or_default().into(),
            oem_bench_mode: value.roller_bench_detection.enum_value_or_default()
                == RollerBenchDetection::ROLLER_BENCH_DETECTION_REQUEST,
            ..Default::default()
        }
    }
}
/// Font  Wiper Mode
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum FrWiperMd {
    /// Not Pressed
    #[default]
    /// OnParkPosition
    OnParkPos,
    /// Intermittent
    Intermittent,
    /// Auto by driver
    AutoByDriver,
    /// Auto by AD
    AutoByAd,
    /// Service Position
    ServicePosition,
    /// High speed
    HighSpeed,
    /// Low Speed
    LowSpeed,
    /// Out Park Position
    OutParkPos,
}
/// Vehicle Power Mode
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum PowerMode {
    #[default]
    /// Traction ON
    TractionOn,
    /// Power ON
    PowerOn,
    /// Live on board
    LifeOnBoard,
    /// Software update
    SwUpdate,
    /// Park
    Park,
    /// Low power
    LowPower,
}

/// Ego Probable Rain Status
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum RainStatus {
    #[default]
    /// No Rain
    NoRain = 0,
    /// Very Low
    VeryLow = 1,
    /// Low
    Low = 2,
    /// Medium Low
    MedLow = 3,
    /// Medium High
    MedHigh = 4,
    /// High
    High = 5,
    /// Heavy
    Heavy = 6,
}

impl From<ProbableRainStatusEgo> for RainStatus {
    fn from(value: ProbableRainStatusEgo) -> Self {
        match value {
            ProbableRainStatusEgo::PRSE_NO_RAIN => Self::NoRain,
            ProbableRainStatusEgo::PRSE_VERY_LOW => Self::VeryLow,
            ProbableRainStatusEgo::PRSE_LOW => Self::Low,
            ProbableRainStatusEgo::PRSE_MED_LOW => Self::MedLow,
            ProbableRainStatusEgo::PRSE_MED_HIGH => Self::MedHigh,
            ProbableRainStatusEgo::PRSE_HIGH => Self::High,
            ProbableRainStatusEgo::PRSE_HEAVY => Self::Heavy,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_vehicle_status::ProbableRainStatusEgo> for RainStatus {
    fn from(value: oem::sdv_adas_ego_ego_vehicle_status::ProbableRainStatusEgo) -> Self {
        use oem::sdv_adas_ego_ego_vehicle_status::ProbableRainStatusEgo;
        match value {
            ProbableRainStatusEgo::PRSE_NO_RAIN => Self::NoRain,
            ProbableRainStatusEgo::PRSE_VERY_LOW => Self::VeryLow,
            ProbableRainStatusEgo::PRSE_LOW => Self::Low,
            ProbableRainStatusEgo::PRSE_MED_LOW => Self::MedLow,
            ProbableRainStatusEgo::PRSE_MED_HIGH => Self::MedHigh,
            ProbableRainStatusEgo::PRSE_HIGH => Self::High,
            ProbableRainStatusEgo::PRSE_HEAVY => Self::Heavy,
        }
    }
}

impl From<FrontWiperMode> for FrWiperMd {
    fn from(value: FrontWiperMode) -> Self {
        match value {
            FrontWiperMode::FWM_AUTO_BY_AD => Self::AutoByAd,
            FrontWiperMode::FWM_AUTO_BY_DRIVER => Self::AutoByDriver,
            FrontWiperMode::FWM_HIGH_SPEED => Self::HighSpeed,
            FrontWiperMode::FWM_LOW_SPEED => Self::LowSpeed,
            FrontWiperMode::FWM_INTERMITTENT => Self::Intermittent,
            FrontWiperMode::FWM_OFF_ON_PARK_POSITION => Self::OnParkPos,
            FrontWiperMode::FWM_OFF_OUT_PARK_POSITION => Self::OutParkPos,
            FrontWiperMode::FWM_SERVICE_POSITION => Self::ServicePosition,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_vehicle_status::FrontWiperMode> for FrWiperMd {
    fn from(value: oem::sdv_adas_ego_ego_vehicle_status::FrontWiperMode) -> Self {
        use oem::sdv_adas_ego_ego_vehicle_status::FrontWiperMode;
        match value {
            FrontWiperMode::FWM_AUTO_BY_AD => Self::AutoByAd,
            FrontWiperMode::FWM_AUTO_BY_DRIVER => Self::AutoByDriver,
            FrontWiperMode::FWM_HIGH_SPEED => Self::HighSpeed,
            FrontWiperMode::FWM_LOW_SPEED => Self::LowSpeed,
            FrontWiperMode::FWM_INTERMITTENT => Self::Intermittent,
            FrontWiperMode::FWM_OFF_ON_PARK_POSITION => Self::OnParkPos,
            FrontWiperMode::FWM_OFF_OUT_PARK_POSITION => Self::OutParkPos,
            FrontWiperMode::FWM_SERVICE_POSITION => Self::ServicePosition,
        }
    }
}

impl From<VehPwrMode> for PowerMode {
    fn from(value: VehPwrMode) -> Self {
        match value {
            VehPwrMode::VPM_TRACTION_ON => Self::TractionOn,
            VehPwrMode::VPM_POWER_ON => Self::PowerOn,
            VehPwrMode::VPM_LIFE_ON_BOARD => Self::LifeOnBoard,
            VehPwrMode::VPM_SW_UPDATE => Self::SwUpdate,
            VehPwrMode::VPM_PARK => Self::Park,
            VehPwrMode::VPM_LOW_POWER => Self::LowPower,
        }
    }
}

#[cfg(feature = "caros")]
impl From<oem::sdv_adas_ego_ego_vehicle_status::VehPwrMode> for PowerMode {
    fn from(value: oem::sdv_adas_ego_ego_vehicle_status::VehPwrMode) -> Self {
        use oem::sdv_adas_ego_ego_vehicle_status::VehPwrMode;
        match value {
            VehPwrMode::VPM_TRACTION_ON => Self::TractionOn,
            VehPwrMode::VPM_POWER_ON => Self::PowerOn,
            VehPwrMode::VPM_LIFE_ON_BOARD => Self::LifeOnBoard,
            VehPwrMode::VPM_SW_UPDATE => Self::SwUpdate,
            VehPwrMode::VPM_PARK => Self::Park,
            VehPwrMode::VPM_LOW_POWER => Self::LowPower,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_veh_status_conversion() {
        let vehicle_status = vehicleStatus_t {
            trailerPresence: true,
            frontWiperStatus: FrontWiperMode::FWM_HIGH_SPEED,
            vehicleStates: VehPwrMode::VPM_POWER_ON,
            dynamicMass: 1500,
            probableRainSts: ProbableRainStatusEgo::PRSE_HEAVY,
            ..Default::default()
        };

        let veh_status: VehStatus = (&vehicle_status).into();

        assert!(veh_status.trailer_presence);
        assert_eq!(veh_status.front_wiper_status, FrWiperMd::HighSpeed);
        assert_eq!(veh_status.pwr_mode, PowerMode::PowerOn);
        assert_eq!(veh_status.dyn_mass, 1500);
        assert_eq!(veh_status.rain_status, RainStatus::Heavy);
    }

    #[test]
    fn test_fr_wiper_md_conversion() {
        assert_eq!(
            FrWiperMd::from(FrontWiperMode::FWM_AUTO_BY_AD),
            FrWiperMd::AutoByAd
        );
        assert_eq!(
            FrWiperMd::from(FrontWiperMode::FWM_AUTO_BY_DRIVER),
            FrWiperMd::AutoByDriver
        );
        assert_eq!(
            FrWiperMd::from(FrontWiperMode::FWM_HIGH_SPEED),
            FrWiperMd::HighSpeed
        );
        assert_eq!(
            FrWiperMd::from(FrontWiperMode::FWM_LOW_SPEED),
            FrWiperMd::LowSpeed
        );
        assert_eq!(
            FrWiperMd::from(FrontWiperMode::FWM_INTERMITTENT),
            FrWiperMd::Intermittent
        );
        assert_eq!(
            FrWiperMd::from(FrontWiperMode::FWM_OFF_ON_PARK_POSITION),
            FrWiperMd::OnParkPos
        );
        assert_eq!(
            FrWiperMd::from(FrontWiperMode::FWM_OFF_OUT_PARK_POSITION),
            FrWiperMd::OutParkPos
        );
        assert_eq!(
            FrWiperMd::from(FrontWiperMode::FWM_SERVICE_POSITION),
            FrWiperMd::ServicePosition
        );
    }

    #[test]
    fn test_power_mode_conversion() {
        assert_eq!(
            PowerMode::from(VehPwrMode::VPM_TRACTION_ON),
            PowerMode::TractionOn
        );
        assert_eq!(
            PowerMode::from(VehPwrMode::VPM_POWER_ON),
            PowerMode::PowerOn
        );
        assert_eq!(
            PowerMode::from(VehPwrMode::VPM_LIFE_ON_BOARD),
            PowerMode::LifeOnBoard
        );
        assert_eq!(
            PowerMode::from(VehPwrMode::VPM_SW_UPDATE),
            PowerMode::SwUpdate
        );
        assert_eq!(PowerMode::from(VehPwrMode::VPM_PARK), PowerMode::Park);
        assert_eq!(
            PowerMode::from(VehPwrMode::VPM_LOW_POWER),
            PowerMode::LowPower
        );
    }

    #[test]
    fn test_rain_status_conversion() {
        assert_eq!(
            RainStatus::from(ProbableRainStatusEgo::PRSE_NO_RAIN),
            RainStatus::NoRain
        );
        assert_eq!(
            RainStatus::from(ProbableRainStatusEgo::PRSE_VERY_LOW),
            RainStatus::VeryLow
        );
        assert_eq!(
            RainStatus::from(ProbableRainStatusEgo::PRSE_LOW),
            RainStatus::Low
        );
        assert_eq!(
            RainStatus::from(ProbableRainStatusEgo::PRSE_MED_LOW),
            RainStatus::MedLow
        );
        assert_eq!(
            RainStatus::from(ProbableRainStatusEgo::PRSE_MED_HIGH),
            RainStatus::MedHigh
        );
        assert_eq!(
            RainStatus::from(ProbableRainStatusEgo::PRSE_HIGH),
            RainStatus::High
        );
        assert_eq!(
            RainStatus::from(ProbableRainStatusEgo::PRSE_HEAVY),
            RainStatus::Heavy
        );
    }
}
