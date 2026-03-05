//! Define ego inputs for ACC
//!
use core::time::Duration;

use super::edc::steering::Steering;
use super::edc::stop::StandStillStatus;
pub use super::edc::{
    brake::Braking, chassis_in_regulation::ChassisInRegulation, ego_dyn::EgoDynamics,
    ego_dyn::EgoMvtStatus, gear::Gear, global_trajectory_status::TrajectoryGlobalStatus,
    lights::Lights, pedal::Pedal, stop::Stop, veh_status::VehStatus,
};
use super::edc::{doors::EgoDoors, engine::Engine};
use super::{mps, mps2, Acceleration, Angle, Velocity};
use acc_interface::datatypes::{belts_t, SafetyBeltBuckle};
#[cfg(feature = "caros")]
use catalog_ampere_adas as oem;
#[allow(unused_imports)]
use num_traits::Float;

/// Ego Data Inputs
#[derive(Debug, Clone, Default, PartialEq)]
pub enum OnOffState {
    /// True
    On,
    /// False
    #[default]
    Off,
}
/// Ego Data Inputs
#[derive(Debug, Clone, Default)]
pub struct Ego {
    /// Timestamp
    pub timestamp: Duration,
    /// Ego dynamics from Scene Understanding / World Model AMS
    pub dynamics: EgoDynamics,
    /// Ego Braking from Ego Data AMS
    pub braking: Braking,
    /// Ego lights from Ego Data AMS
    pub lights: Lights,
    /// Ego chassis in regulation from Ego Data AMS
    pub chassis_in_reg: ChassisInRegulation,
    /// Ego vehicle state from Ego Data AMS
    pub vehicle_state: VehStatus,
    /// Is one of Ego Doors Open
    pub doors: EgoDoors,
    /// Driver Belt Fastened
    pub driver_belt_fastened: OnOffState,
    /// Stop Status
    pub stop: Stop,
    /// ture if braking ramp reaches its end
    pub end_braking_ramp: bool,
    /// Pedal Status
    pub pedal: Pedal,
    /// Slope Estimation
    pub slope: Angle,
    /// Ego Gear
    pub gear: Gear,
    /// ego engine
    pub engine: Engine,
    /// Trajectory Global Status
    pub trajectory_global_status: TrajectoryGlobalStatus,
    /// Steering
    pub steering: Steering,
}
impl From<bool> for OnOffState {
    fn from(value: bool) -> Self {
        if value {
            OnOffState::On
        } else {
            OnOffState::Off
        }
    }
}

impl From<&belts_t> for OnOffState {
    fn from(belts: &belts_t) -> Self {
        if matches!(belts.driverSafetyBeltBuckle, SafetyBeltBuckle::SBB_FASTENED) {
            OnOffState::On
        } else {
            OnOffState::Off
        }
    }
}
#[cfg(feature = "caros")]
impl From<&oem::sdv_adas_ego_ego_doors_and_belts::Belts_t> for OnOffState {
    fn from(belts: &oem::sdv_adas_ego_ego_doors_and_belts::Belts_t) -> Self {
        use oem::sdv_adas_ego_ego_doors_and_belts::SafetyBeltBuckle::*;
        if matches!(
            belts.driver_safety_belt_buckle.enum_value(),
            Ok(SBB_FASTENED)
        ) {
            OnOffState::On
        } else {
            OnOffState::Off
        }
    }
}

impl Ego {
    /// detect the moment that braking ramp is finished
    fn end_braking_ramp_detection(&mut self, standstill_was_off: bool) {
        self.end_braking_ramp = standstill_was_off
            && matches!(self.stop.global_standstill_status, StandStillStatus::Hold);
    }

    /// update ego steering
    pub fn update_steering(&mut self, steering: Steering) {
        self.steering = steering;
    }
    /// Update ego dynamics
    pub fn update_dynamics(&mut self, ego_dyn: EgoDynamics) {
        self.dynamics = ego_dyn;
    }
    /// Update ego braking
    pub fn update_braking(&mut self, ego_braking: Braking) {
        self.braking = ego_braking;
    }
    /// Update ego lights
    pub fn update_lights(&mut self, ego_lights: Lights) {
        use super::edc::lights::OnOffState;
        let prev = &self.lights.blinker_status;
        let overtaking_confirmation = (ego_lights.blinker_status.eq(&OnOffState::LeftOn)
            && prev.ne(&OnOffState::LeftOn))
            || (ego_lights.blinker_status.eq(&OnOffState::RightOn)
                && prev.ne(&OnOffState::RightOn));
        self.lights = ego_lights;
        self.lights.overtaking_confirmation = overtaking_confirmation;
    }
    /// Update chassis in regulation
    pub fn update_chassis(&mut self, ego_chassis: ChassisInRegulation) {
        self.chassis_in_reg = ego_chassis;
    }
    /// Update ego state
    pub fn update_veh_state(&mut self, veh_state: VehStatus) {
        let mode_changed = self.vehicle_state.oem_bench_mode != veh_state.oem_bench_mode;
        self.vehicle_state = veh_state;
        self.vehicle_state.bench_mode_switch = mode_changed;
    }
    /// Update ego door
    pub fn update_door(&mut self, doors: EgoDoors) {
        self.doors = doors;
    }
    /// Update ego belts
    pub fn update_belts(&mut self, belt: OnOffState) {
        self.driver_belt_fastened = belt;
    }
    /// Update ego stop state
    pub fn update_stop(&mut self, stop: Stop) {
        let prev_gss = self.stop.global_standstill_status.clone();
        self.stop = stop;
        self.stop.prev_global_standstill_status = prev_gss;
        self.end_braking_ramp_detection(
            self.stop
                .prev_global_standstill_status
                .eq(&StandStillStatus::NotHold),
        );
    }
    /// Update ego pedal
    pub fn update_pedal(&mut self, pedal: Pedal) {
        self.pedal = pedal;
    }
    /// Update ego slope
    pub fn update_slope(&mut self, slope: Angle) {
        self.slope = slope;
    }
    /// Update ego gear
    pub fn update_gear(&mut self, gear: Gear) {
        self.gear = gear;
    }
    /// Update ego engine
    pub fn update_engine(&mut self, engine: Engine) {
        self.engine = engine;
    }
    /// Update trajectory global status
    pub fn update_trajectory_global_status(
        &mut self,
        trajectory_global_status: TrajectoryGlobalStatus,
    ) {
        self.trajectory_global_status = trajectory_global_status;
    }
}

impl EgoDynamics {
    /// Compute signed speed
    pub fn signed_speed(&self) -> Velocity {
        let vx = self.vx.get::<mps>().powi(2);
        let vy = self.vy.get::<mps>().powi(2);
        let mut speed = (vx + vy).sqrt();
        if self.mv_status.eq(&EgoMvtStatus::MvBackward) {
            speed *= -1.0;
        }
        Velocity::new::<mps>(speed)
    }
    /// Compute signed acceleration
    pub fn signed_accel(&self) -> Acceleration {
        let ax = self.ax.get::<mps2>().powi(2);
        let ay = self.ay.get::<mps2>().powi(2);
        let mut accel = (ax + ay).sqrt();
        if self.mv_status.eq(&EgoMvtStatus::MvBackward) {
            accel *= -1.0;
        }
        Acceleration::new::<mps2>(accel)
    }
    /// Ego is stopped
    pub fn is_ego_stopped(&self) -> bool {
        matches!(self.mv_status, EgoMvtStatus::StMoving)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::io::deg as degree;
    use crate::io::edc::global_trajectory_status::StandStillReq;
    use crate::io::rad as radian;
    use acc_interface::datatypes::{
        braking_t, chassisInRegulation_t, doors_t, egodynamics_t, gear_t, lighting_t, pedals_t,
        stop_t, vehicleStatus_t, AccelPedalKickDown, BlinkersStatus, BrakeInfoStatusEgo,
        BrakePedal, DataQualifier, DoorState, EgoMovementStatus, FlashingIndicatorStatus,
        FrontWiperMode, GearLeverPosition, ParkingBrakeRequestInProgressEgo, ProbableRainStatusEgo,
        StandStillStatusEgo, TailGateStatus, TurnSignalCmdSte, VehPwrMode,
    };

    #[test]
    fn test_ego_initialization() {
        let ego_braking: Braking = (&braking_t {
            infoStatus: BrakeInfoStatusEgo::BISE_BRAKE_PEDAL_CONFIRMED_PRESSED,
            brakeWhlTqEstimation: 100,
            ..Default::default()
        })
            .into();

        let ego_gear: Gear = (&gear_t {
            rear: true,
            gearLeverPosition: GearLeverPosition::GLP_DRIVE,
            ..Default::default()
        })
            .into();

        let ego_lights: Lights = (&lighting_t {
            flashingIndicatorPosition: TurnSignalCmdSte::TSCS_LEFT,
            flashingIndicatorStatus: FlashingIndicatorStatus::FIS_LEFT_ON_RIGHT_ON,
            blinkersStatus: BlinkersStatus::BLINKERS_STATUS_LEFT_OFF_RIGHT_OFF,
        })
            .into();

        let ego_pedal: Pedal = (&pedals_t {
            accelPedalKickDown: AccelPedalKickDown::APKD_ACTIVATED,
            brakePedal: BrakePedal::BRAKE_PEDAL_PRESSED,
            ..Default::default()
        })
            .into();

        let ego_stop: Stop = (&stop_t {
            globalStandstillStatus: StandStillStatusEgo::SSSE_VEHICLE_HOLD,
            parkingBrakeRequestInProgress:
                ParkingBrakeRequestInProgressEgo::PBRIPE_BRAKE_TIGHTENING_REQUEST,
            ..Default::default()
        })
            .into();

        let ego_vehicle_state: VehStatus = (&vehicleStatus_t {
            trailerPresence: true,
            frontWiperStatus: FrontWiperMode::FWM_HIGH_SPEED,
            vehicleStates: VehPwrMode::VPM_POWER_ON,
            dynamicMass: 1500,
            probableRainSts: ProbableRainStatusEgo::PRSE_HEAVY,
            ..Default::default()
        })
            .into();

        let ego_dynamics: EgoDynamics = (
            &egodynamics_t {
                velocity: [12.0, -3.0, 0.0],
                acceleration: [1.5, -0.5, 0.0],
                movementstatus: EgoMovementStatus::EMS_MOVING_FORWARD,
                ..Default::default()
            },
            &DataQualifier::DQ_NORMAL,
        )
            .into();

        let ego_chassis_in_reg: ChassisInRegulation = (&chassisInRegulation_t {
            tcsDeactivated: acc_interface::datatypes::TcsDeactivatedEgo::TDE_AVAILABLE,
            aycDeactivated: acc_interface::datatypes::AycDeactivatedEgo::ADE_AVAILABLE,
            absInRegulation: true,
            asrInRegulation: false,
            aycInRegulation: false,
            msrInRegulation: true,
            tsfInRegulation: false,
        })
            .into();

        let ego_slope = Angle::new::<degree>(30.0);

        let _ego = Ego {
            dynamics: ego_dynamics,
            braking: ego_braking,
            lights: ego_lights,
            chassis_in_reg: ego_chassis_in_reg,
            vehicle_state: ego_vehicle_state,
            doors: EgoDoors::default(),
            driver_belt_fastened: OnOffState::Off,
            stop: ego_stop,
            end_braking_ramp: false,
            pedal: ego_pedal,
            slope: ego_slope,
            gear: ego_gear,
            ..Default::default()
        };
        // TODO : Add checks
    }
    #[test]
    fn test_from_doors_t_to_on_off_state() {
        let mut doors = doors_t {
            driverDoorState: DoorState::DOOR_STATE_NOT_AVAILBLE,
            passengerDoorStateFront: DoorState::DOOR_STATE_DOOR_CLOSED,
            passengerDoorStateFL: DoorState::DOOR_STATE_DOOR_OPEN,
            passengerDoorStateFR: DoorState::DOOR_STATE_NOT_USED,
            passengerDoorStateRR: DoorState::DOOR_STATE_DOOR_CLOSED,
            passengerDoorStateRL: DoorState::DOOR_STATE_DOOR_OPEN,
            unclosedDoorModeState: false,
            tailGateStatus: TailGateStatus::TGS_TAILGATE_IS_CLOSED,
        };
        let door: EgoDoors = (&doors).into();
        assert_eq!(door.rl_door_state, OnOffState::On);
        assert_eq!(door.rr_door_state, OnOffState::Off);
        doors.passengerDoorStateFL = DoorState::DOOR_STATE_DOOR_CLOSED;
        doors.passengerDoorStateRL = DoorState::DOOR_STATE_DOOR_CLOSED;
        let door: EgoDoors = (&doors).into();
        assert_eq!(door.rl_door_state, OnOffState::Off);
        assert_eq!(door.rr_door_state, OnOffState::Off);
    }
    #[test]
    fn test_from_belts_t_to_on_off() {
        let mut belts = belts_t {
            driverSafetyBeltBuckle: SafetyBeltBuckle::SBB_FASTENED,
            ..Default::default()
        };
        assert_eq!(OnOffState::from(&belts), OnOffState::On);
        belts.driverSafetyBeltBuckle = SafetyBeltBuckle::SBB_UNFASTENED;
        assert_eq!(OnOffState::from(&belts), OnOffState::Off);
    }
    #[test]
    fn test_end_braking_ramp() {
        let mut ego = Ego::default();
        ego.stop.global_standstill_status = StandStillStatus::Hold;
        let standstill_off = true;
        ego.end_braking_ramp_detection(standstill_off);
        assert!(ego.end_braking_ramp);
        ego.stop.global_standstill_status = StandStillStatus::HoldNotPossible;
        ego.end_braking_ramp_detection(standstill_off);
        assert!(!ego.end_braking_ramp);
    }
    #[test]
    fn test_update_ego_dyn() {
        let mut ego = Ego::default();
        let dynamic = EgoDynamics {
            vx: Velocity::new::<mps>(14.2),
            ..Default::default()
        };
        ego.update_dynamics(dynamic);
        assert_eq!(Velocity::new::<mps>(14.2), ego.dynamics.vx);
    }
    #[test]
    fn test_update_ego_braking() {
        let mut ego = Ego::default();
        let braking = Braking {
            whl_tq_estimation: 41,
            ..Default::default()
        };
        ego.update_braking(braking);
        assert_eq!(ego.braking.whl_tq_estimation, 41);
    }
    #[test]
    fn test_update_ego_lights() {
        let mut ego = Ego::default();
        let mut lights = Lights::default();
        lights.blinker_status = crate::io::edc::lights::OnOffState::RightOn;
        ego.update_lights(lights);
        assert_eq!(
            ego.lights.blinker_status,
            crate::io::edc::lights::OnOffState::RightOn
        );
    }
    #[test]
    fn test_update_ego_chassis() {
        let mut ego = Ego::default();
        let chassis = ChassisInRegulation {
            esc_incontrol: true,
            ..Default::default()
        };
        ego.update_chassis(chassis);
        assert!(ego.chassis_in_reg.esc_incontrol);
    }
    #[test]
    fn test_update_ego_state() {
        let mut ego = Ego::default();
        let veh_state = VehStatus {
            dyn_mass: 1458,
            ..Default::default()
        };
        ego.update_veh_state(veh_state);
        assert_eq!(ego.vehicle_state.dyn_mass, 1458);
    }
    #[test]
    fn test_update_ego_door() {
        let mut ego = Ego::default();
        let doors = doors_t {
            passengerDoorStateRL: DoorState::DOOR_STATE_DOOR_OPEN,
            unclosedDoorModeState: true,
            ..Default::default()
        };
        let door: EgoDoors = (&doors).into();
        ego.update_door(door);
        assert_eq!(ego.doors.rl_door_state, OnOffState::On);
        assert_eq!(ego.doors.unclosed_door_mode, OnOffState::On);
    }
    #[test]
    fn test_update_ego_belts() {
        let mut ego = Ego::default();
        let belts = belts_t {
            driverSafetyBeltBuckle: SafetyBeltBuckle::SBB_FASTENED,
            ..Default::default()
        };
        let belt_fastened: OnOffState = (&belts).into();
        ego.update_belts(belt_fastened);
        assert_eq!(ego.driver_belt_fastened, OnOffState::On);
    }
    #[test]
    fn test_update_ego_stop() {
        let mut ego = Ego::default();
        let stop = Stop {
            is_roll_over_detected: true,
            ..Default::default()
        };
        ego.update_stop(stop);
        assert!(ego.stop.is_roll_over_detected);
    }
    #[test]
    fn test_update_ego_pedal() {
        let mut ego = Ego::default();
        let pedal = Pedal {
            accel_pedal: 158.3,
            ..Default::default()
        };
        ego.update_pedal(pedal);
        assert_eq!(ego.pedal.accel_pedal, 158.3);
    }
    #[test]
    fn test_update_ego_slope() {
        let mut ego = Ego::default();
        let slope = Angle::new::<radian>(14.2);
        ego.update_slope(slope);
        assert_eq!(ego.slope, slope);
    }
    #[test]
    fn test_update_ego_gear() {
        let mut ego = Ego::default();
        let gear = Gear {
            is_rear_gear_engaged: true,
            ..Default::default()
        };
        ego.update_gear(gear);
        assert!(ego.gear.is_rear_gear_engaged);
    }
    #[test]
    fn test_update_ego_engine() {
        let mut ego = Ego::default();
        let engine = Engine {
            driver_override: false,
            ..Default::default()
        };
        ego.update_engine(engine);
        assert!(!ego.engine.driver_override);
    }
    #[test]
    fn test_signed_speed() {
        let mut ego_dyn = EgoDynamics {
            mv_status: EgoMvtStatus::MvBackward,
            vx: Velocity::new::<mps>(2.0),
            vy: Velocity::new::<mps>(2.0),
            ..Default::default()
        };
        let mut velocity = ego_dyn.signed_speed().get::<mps>();
        assert!(velocity.lt(&0.0));

        ego_dyn = EgoDynamics {
            mv_status: EgoMvtStatus::MvForward,
            vx: Velocity::new::<mps>(2.0),
            vy: Velocity::new::<mps>(2.0),
            ..Default::default()
        };
        velocity = ego_dyn.signed_speed().get::<mps>();
        assert!(velocity.gt(&0.0));
    }
    #[test]
    fn test_signed_accel() {
        let ego_dyn = EgoDynamics {
            mv_status: EgoMvtStatus::MvBackward,
            ax: Acceleration::new::<mps2>(2.0),
            ay: Acceleration::new::<mps2>(2.0),
            ..Default::default()
        };
        let accel = ego_dyn.signed_accel().get::<mps2>();
        assert!(accel.lt(&0.0));
    }
    #[test]
    fn test_update_trajectory_global_status() {
        let mut ego = Ego::default();
        let trajectory_global_status = TrajectoryGlobalStatus {
            pwt_wheel_torque: 14.0,
            ..Default::default()
        };
        ego.update_trajectory_global_status(trajectory_global_status);
        assert_eq!(ego.trajectory_global_status.pwt_wheel_torque, 14.0);
        assert_eq!(
            ego.trajectory_global_status.standstill_req,
            StandStillReq::FunctionOff
        );
    }
    #[test]
    fn test_update_bench_mode() {
        let mut ego = Ego::default();
        let mut vehicle_st = VehStatus {
            oem_bench_mode: true,
            ..Default::default()
        };
        ego.update_veh_state(vehicle_st.clone());
        assert!(ego.vehicle_state.oem_bench_mode);
        assert!(ego.vehicle_state.bench_mode_switch);
        ego.update_veh_state(vehicle_st.clone());
        assert!(!ego.vehicle_state.bench_mode_switch);
        vehicle_st.oem_bench_mode = false;
        ego.update_veh_state(vehicle_st);
        assert!(ego.vehicle_state.bench_mode_switch);
        assert!(!ego.vehicle_state.oem_bench_mode);
    }
}
