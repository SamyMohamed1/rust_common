//! Define Config inputs for ACC
//!
use acc_interface::datatypes::{
    BusDT_DiagControlActivation, BusDT_VariantManagement, StateGearbox,
};
use num_derive::FromPrimitive;
use num_traits::cast::FromPrimitive;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, FromPrimitive, Default)]
/// Type of the vehicule System
pub enum VehSysType {
    #[default]
    /// Unknown
    Unknown,
    /// Not Used
    NotUsed,
    /// 4WD
    FourWd,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
/// Type of the vehicule System
pub enum GearBoxType {
    #[default]
    /// Manual
    Manual,
    /// Automatic
    Automatic,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
/// Type of the egine system
pub enum EngineType {
    #[default]
    /// Manual
    Classic,
    /// Phev
    Ev,
    /// Phev
    Hev,
    /// Phev
    PHEV,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
/// Type of the vehicule System
pub enum BrakeSysType {
    #[default]
    /// Classic
    Classic,
    /// Ibooster
    Ibooster,
}

/// Variant management
#[derive(Debug, Clone, Default)]
pub struct VariantMgt {
    /// Gear Box Type
    pub gear_box_type: GearBoxType,
    // /// Engine Type
    // pub engine_type: EngineType,
    /// EPKB
    pub epkb: bool,
    /// Lca Button Pressed
    pub lca_button: bool,
    /// RSL Feature Mgt
    pub rsl_feature: bool,
    /// Vehicule System Type
    pub veh_sys_type: VehSysType,
    // /// Brake System Type
    // pub brake_sys_type: BrakeSysType,
    /// C1AHS Archietcture Feature Mgt
    pub c1ahs_activated: bool,
    /// AD1Enhn Autorestart Feature Mngt
    pub autorestart: bool,
}

impl From<StateGearbox> for GearBoxType {
    fn from(value: StateGearbox) -> Self {
        match value {
            StateGearbox::Manual_Gearbox => Self::Manual,
            StateGearbox::Automatic_Gearbox => Self::Automatic,
        }
    }
}

impl From<BusDT_VariantManagement> for VariantMgt {
    fn from(val: BusDT_VariantManagement) -> Self {
        Self {
            gear_box_type: StateGearbox::from_u8(val.V_x_GearBoxType)
                .unwrap_or_default()
                .into(),
            // engine_type: StateEngineType::from_u8(val.V_x_EngineType)
            //     .unwrap_or_default()
            //     .into(),
            // brake_sys_type: StateBrkSysTyp::from_u8(val.V_x_BrakeSystemType)
            //     .unwrap_or_default()
            //     .into(),
            veh_sys_type: VehSysType::from_u8(val.V_x_VehDriveSystemType).unwrap_or_default(),
            epkb: val.V_x_EPKBManagement,
            lca_button: val.V_x_LCAButtonPresent,
            rsl_feature: val.V_x_RSLFeatureMngt,
            c1ahs_activated: val.V_x_C1AHSArchitectureFeatureMngt,
            autorestart: val.V_x_AD1Enh_AutorestartFeatureMngt,
        }
    }
}
/// Ego Data Inputs
#[derive(Debug, Clone, Default)]
pub struct DiagnosticAct {
    /// Gear Box Type
    pub oem: bool,
    /// Engine Type
    pub inhibition_order: bool,
}

impl From<BusDT_DiagControlActivation> for DiagnosticAct {
    fn from(val: BusDT_DiagControlActivation) -> Self {
        Self {
            oem: val.F_x_GADE_OEMDiag_Activation,
            inhibition_order: val.F_x_GADE_InhibitionOrder,
        }
    }
}

#[cfg(test)]
mod tests {
    use acc_interface::datatypes::{
        BusDT_DiagControlActivation, BusDT_VariantManagement, StateGearbox,
    };

    use crate::io::config::{GearBoxType, VehSysType};

    use super::{DiagnosticAct, VariantMgt};

    #[test]
    fn test_from_stategearbox_to_gearboxtype() {
        let mut state_gear_box = StateGearbox::Automatic_Gearbox;
        let gear_box_type: GearBoxType = state_gear_box.into();
        assert_eq!(gear_box_type, GearBoxType::Automatic);
        state_gear_box = StateGearbox::Manual_Gearbox;
        let gear_box_type: GearBoxType = state_gear_box.into();
        assert_eq!(gear_box_type, GearBoxType::Manual);
    }
    #[test]
    fn test_from_variant_management_to_variantmngt() {
        let variant_management = BusDT_VariantManagement {
            V_x_GearBoxType: 0,
            V_x_EngineType: 2,
            V_x_BrakeSystemType: 1,
            V_x_VehDriveSystemType: 2,
            V_x_EPKBManagement: false,
            V_x_LCAButtonPresent: true,
            V_x_RSLFeatureMngt: false,
            V_x_C1AHSArchitectureFeatureMngt: false,
            V_x_AD1Enh_AutorestartFeatureMngt: true,
            ..Default::default()
        };
        let variant_mngt: VariantMgt = variant_management.into();
        assert_eq!(variant_mngt.gear_box_type, GearBoxType::Manual);
        assert_eq!(variant_mngt.veh_sys_type, VehSysType::FourWd);
        assert!(!variant_mngt.epkb);
        assert!(variant_mngt.lca_button);
        assert!(!variant_mngt.rsl_feature);
        assert!(!variant_mngt.c1ahs_activated);
        assert!(variant_mngt.autorestart);
    }
    #[test]
    fn from_diag_control_activation_to_diagnositcact() {
        let diag_control_activation = BusDT_DiagControlActivation {
            F_x_GADE_OEMDiag_Activation: true,
            F_x_GADE_InhibitionOrder: false,
        };
        let diagnostic_act: DiagnosticAct = diag_control_activation.into();
        assert!(diagnostic_act.oem);
        assert!(!diagnostic_act.inhibition_order);
    }
}
