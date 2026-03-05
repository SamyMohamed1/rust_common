//! Define ego Chassis in Regulation Status for ACC
//!
use acc_interface::datatypes::{chassisInRegulation_t, AycDeactivatedEgo, TcsDeactivatedEgo};
#[cfg(feature = "caros")]
use catalog_ampere_adas as oem;

///  VDC activation
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccVdcState {
    #[default]
    /// VDC Deactivated = available  for Acc App transition Off to On
    Available,
    /// VDC Deactivated = disable for Acc App transition On to Off
    Disable,
}

/// Define Chassis In regulation Inhibition
#[derive(Debug, Default, Clone)]
pub struct ChassisInRegulation {
    /// VDC state for shared transition On/Off
    pub vdc_state: AccVdcState,
    /// ESC in control shared transition
    pub esc_incontrol: bool,
    /// TSF in regulation
    pub tsf_in_regulation: bool,
    /// MSR in regulation
    pub msr_in_regulation: bool,
    /// ASR in regulation
    pub asr_in_regulation: bool,
    /// AYC in regulation
    pub ayc_in_regulation: bool,
    /// ABS in regulation
    pub abs_in_regulation: bool,
}

impl From<&chassisInRegulation_t> for ChassisInRegulation {
    fn from(value: &chassisInRegulation_t) -> Self {
        use AycDeactivatedEgo::*;
        use TcsDeactivatedEgo::*;
        let vdc_state = if matches!(value.aycDeactivated, ADE_DISABLE)
            || matches!(value.tcsDeactivated, TDE_DISABLE)
        {
            AccVdcState::Disable
        } else {
            AccVdcState::Available
        };
        let esc_incontrol = value.absInRegulation
            || value.asrInRegulation
            || value.aycInRegulation
            || value.msrInRegulation
            || value.tsfInRegulation;

        Self {
            vdc_state,
            esc_incontrol,
            tsf_in_regulation: value.tsfInRegulation,
            msr_in_regulation: value.msrInRegulation,
            asr_in_regulation: value.asrInRegulation,
            ayc_in_regulation: value.aycInRegulation,
            abs_in_regulation: value.absInRegulation,
        }
    }
}

#[cfg(feature = "caros")]
impl From<&oem::sdv_adas_ego_ego_chassis::ChassisInRegulation_t> for ChassisInRegulation {
    fn from(value: &oem::sdv_adas_ego_ego_chassis::ChassisInRegulation_t) -> Self {
        use oem::sdv_adas_ego_ego_chassis::AycDeactivatedEgo::*;
        use oem::sdv_adas_ego_ego_chassis::TcsDeactivatedEgo::*;
        let vdc_state = if matches!(value.ayc_deactivated.enum_value(), Ok(ADE_DISABLE))
            || matches!(value.tcs_deactivated.enum_value(), Ok(TDE_DISABLE))
        {
            AccVdcState::Disable
        } else {
            AccVdcState::Available
        };
        let esc_incontrol = value.abs_in_regulation
            || value.asr_in_regulation
            || value.ayc_in_regulation
            || value.msr_in_regulation
            || value.tsf_in_regulation;

        Self {
            vdc_state,
            esc_incontrol,
            tsf_in_regulation: value.tsf_in_regulation,
            msr_in_regulation: value.msr_in_regulation,
            asr_in_regulation: value.asr_in_regulation,
            ayc_in_regulation: value.ayc_in_regulation,
            abs_in_regulation: value.abs_in_regulation,
        }
    }
}

#[cfg(test)]
mod test {
    use acc_interface::datatypes::{chassisInRegulation_t, AycDeactivatedEgo};

    use crate::io::edc::chassis_in_regulation::AccVdcState;

    use super::ChassisInRegulation;
    #[test]
    fn test_from_chassis_in_regulation() {
        let mut chassis_in_regulation_t = chassisInRegulation_t {
            asrInRegulation: true,
            aycDeactivated: AycDeactivatedEgo::ADE_DISABLE,
            ..Default::default()
        };
        let chassis_in_reg: ChassisInRegulation = (&chassis_in_regulation_t).into();
        assert_eq!(chassis_in_reg.vdc_state, AccVdcState::Disable);
        assert!(chassis_in_reg.esc_incontrol);
        chassis_in_regulation_t = chassisInRegulation_t {
            aycInRegulation: true,
            ..Default::default()
        };
        let chassis_in_reg: ChassisInRegulation = (&chassis_in_regulation_t).into();
        assert!(chassis_in_reg.esc_incontrol);
        chassis_in_regulation_t = chassisInRegulation_t {
            msrInRegulation: true,
            ..Default::default()
        };
        let chassis_in_reg: ChassisInRegulation = (&chassis_in_regulation_t).into();
        assert!(chassis_in_reg.esc_incontrol);
        chassis_in_regulation_t = chassisInRegulation_t {
            tsfInRegulation: true,
            ..Default::default()
        };
        let chassis_in_reg: ChassisInRegulation = (&chassis_in_regulation_t).into();
        assert!(chassis_in_reg.esc_incontrol);
    }
}
