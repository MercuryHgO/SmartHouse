use std::fmt::Display;

use super::{GaugeName, GaugeState, Gauge};

pub const FIRE_ALARM_ID: &[u8] = "fire_alarm".as_bytes();

#[derive(Debug)]
pub enum FireAlarmState {
    Disabled,
    Enabled,
    OnAlert
}

impl GaugeState for FireAlarmState {
    fn parse_state(state: super::SerializedState) -> crate::Result<Self> {
        let state = match state[0] {
            0 => FireAlarmState::Disabled,
            1 => FireAlarmState::Enabled,
            2 => FireAlarmState::OnAlert,
            _ => Err("Error deserializing state")?
        };

        Ok(state)
    }
}

pub struct FireAlarm {
    name: GaugeName,
    state: FireAlarmState
}

impl Display for FireAlarm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.state {
            FireAlarmState::Disabled => 
                write!(f,"{} disabled",self.name),
            FireAlarmState::Enabled => 
                write!(f,"{} enabled",self.name),
            FireAlarmState::OnAlert => 
                write!(f,"{} catched fire",self.name),
        }
    }
}

impl Gauge for FireAlarm {
    type GaugeState = FireAlarmState;

    fn new(name: GaugeName, state: Self::GaugeState) -> Self {
        Self { name, state }
    }

    fn serialize_id(&self) -> super::SerializedGaugeId {
        (FIRE_ALARM_ID.len(),FIRE_ALARM_ID.to_vec())
    }

    fn serialize_name(&self) -> super::SerializedGaugeName {
        (self.name.len(),self.name.clone().into())
    }

    fn serialize_state(&self) -> super::SerializedGaugeState {
        let state = match self.state {
            FireAlarmState::Disabled => 0,
            FireAlarmState::Enabled => 1,
            FireAlarmState::OnAlert => 2,
        };

        (state,None)
    }

    fn state(&self) -> &FireAlarmState {
        &self.state
    }

    fn name(&self) -> &GaugeName {
        &self.name
    }

    fn id() -> super::GaugeIdentifier {
        FIRE_ALARM_ID.to_vec()
    }

    fn set_state(&mut self, state: Self::GaugeState) {
        self.state = state
    }

    fn set_name(&mut self, name: GaugeName) {
        self.name = name
    }
}
