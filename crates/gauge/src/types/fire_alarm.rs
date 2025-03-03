use std::fmt::Display;

use super::{GaugeName, GaugeState, Gauge};

pub const FIRE_ALARM_ID: &[u8] = "fire_alarm".as_bytes();

#[derive(Debug, PartialEq, Eq)]
pub enum FireAlarmState {
    Disabled,
    Enabled,
    OnAlert
}

impl Display for FireAlarmState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            FireAlarmState::Disabled => "Disabled",
            FireAlarmState::Enabled => "Enabled",
            FireAlarmState::OnAlert => "On alert",
        };

        write!(f,"{}",message)
    }
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

#[derive(Debug, PartialEq, Eq)]
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
        FIRE_ALARM_ID.to_vec()
    }

    fn serialize_name(&self) -> super::SerializedGaugeName {
        self.name.clone().into()
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

#[cfg(test)]
mod test {
    use crate::types::Gauge;

    use super::{FireAlarm, FireAlarmState};

    #[test]
    fn serialize_and_deserialize() {
        let fire_alarm: FireAlarm = FireAlarm::new("Room".to_string(),FireAlarmState::Enabled);

        let serialized = fire_alarm.serialize();

        let deserialized = FireAlarm::deserialize(serialized.into())
            .expect("Deserialization error");

        assert_eq!(fire_alarm,deserialized)

    }
}
