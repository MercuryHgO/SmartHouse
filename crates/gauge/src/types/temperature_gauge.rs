use std::fmt::Display;

use super::{GaugeIdentifier, GaugeState, Gauge};

pub const TEMPERATURE_GAUGE_ID: &[u8] = "temperature_gauge".as_bytes();

type Temperature = f32;

#[derive(Debug)]
pub enum TemperatureGaugeState {
    Disabled,
    Enabled,
    ReadedTemperarure(Temperature)
}

impl GaugeState for TemperatureGaugeState {
    fn parse_state(state: super::SerializedState) -> crate::Result<Self> {
        let state = match state[0] {
            0 => TemperatureGaugeState::Disabled,
            1 => TemperatureGaugeState::Enabled,
            2 => {
                let temp_as_bytes: [u8;4]  = state[2..6].try_into()?;
                let temp = f32::from_be_bytes(temp_as_bytes);
                TemperatureGaugeState::ReadedTemperarure(temp)
            },
            _ => Err("Unknown Temperature Gauge State")?
        };

        Ok(state)
    }
}

#[derive(Debug)]
pub struct TemperatureGauge {
    name: String,
    state: TemperatureGaugeState
}

impl Gauge for TemperatureGauge {
    type GaugeState = TemperatureGaugeState;

    fn new(name: super::GaugeName, state: Self::GaugeState) -> Self {
        Self { name, state }
    }

    fn state(&self) -> &Self::GaugeState {
        &self.state
    }

    fn set_state(&mut self, state: Self::GaugeState) {
        self.state = state
    }

    fn name(&self) -> &super::GaugeName {
        &self.name
    }

    fn set_name(&mut self, name: super::GaugeName) {
        self.name = name
    }

    fn id() -> GaugeIdentifier {
        TEMPERATURE_GAUGE_ID.to_vec()
    }

    fn serialize_id(&self) -> super::SerializedGaugeId {
        (TEMPERATURE_GAUGE_ID.len(),TEMPERATURE_GAUGE_ID.to_vec())
    }

    fn serialize_name(&self) -> super::SerializedGaugeName {
        (self.name.len(),self.name.as_bytes().to_vec())
    }

    fn serialize_state(&self) -> super::SerializedGaugeState {
        match self.state {
            TemperatureGaugeState::Disabled => (0,None),
            TemperatureGaugeState::Enabled =>  (1,None),
            TemperatureGaugeState::ReadedTemperarure(temp) => {
                let temp_as_bytes = temp.to_be_bytes();
                (2,Some((4 as usize,temp_as_bytes.to_vec())))
            },
        }
    }
}

impl Display for TemperatureGauge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.state {
            TemperatureGaugeState::Disabled => {
                write!(f, "{} temperature gauge disabled", self.name)
            },
            TemperatureGaugeState::Enabled => {
                write!(f, "{} temperature gauge enabled", self.name)
            },
            TemperatureGaugeState::ReadedTemperarure(temp) => {
                write!(f, "'{}' temperature: {}", self.name, temp)
            },
        }
    }
}
