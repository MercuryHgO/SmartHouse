type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T,Error>;

pub type IsUpdated = bool;
pub type GaugeName = String;
pub type GaugeAsBytes = Vec<u8>;

#[derive(Debug)]
pub enum GaugeState {
    Disabled,
    Enabled,
    Message(String)
}

#[derive(Debug)]
pub struct DeserializedGauge {
    name: GaugeName,
    state: GaugeState
}

impl DeserializedGauge {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn state(&self) -> &GaugeState {
        &self.state
    }
}

pub fn deserialize(bytes: &[u8]) -> Result<DeserializedGauge> {
    let name_length = bytes[0] as usize;
    let name_slice = &bytes[1..1+name_length];
    let name = String::from_utf8(name_slice.into())?;

    let state_byte = bytes[name_length+1];

    let state: GaugeState = match state_byte {
        0 => GaugeState::Disabled,
        1 => GaugeState::Enabled,
        2 => {
            let state_message_length = bytes[name_length+2] as usize;
            let message_slice= &bytes[3+name_length..3+name_length+state_message_length];

            GaugeState::Message(String::from_utf8(message_slice.to_vec())?)
        },
        _ => Err("Unexpected state byte")?
    };

    Ok(DeserializedGauge {
        name, state
    })
}

pub trait Gauge {
    fn name(&self) -> &str;
    fn state(&self) -> &GaugeState;

    fn set_state(&mut self, state: GaugeState);

    fn is_updated(&self) -> IsUpdated;
    fn set_is_updated(&mut self, is_updated: IsUpdated);

    /// Checks is gauge updated, and sets updated state to `false`
    fn fetch_update(&mut self) -> IsUpdated {
        if self.is_updated() {
            self.set_is_updated(false);
            return true
        }
        false
    }

    fn serialize(&self) -> GaugeAsBytes {
        let mut buffer = Vec::<u8>::new();
        let name_bytes = self.name().as_bytes();

        buffer.push(name_bytes.len() as u8);
        buffer.extend_from_slice(name_bytes);

        match self.state() {
            GaugeState::Disabled => {
                buffer.push(0)
            },
            GaugeState::Enabled => {
                buffer.push(1)
            },
            GaugeState::Message(msg) => {
                buffer.push(2);

                let msg_bytes = msg.as_bytes();
                buffer.push(msg_bytes.len() as u8);
                buffer.extend_from_slice(msg_bytes);
            },
        }

        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct FireAlarm {
        name: String,
        state: GaugeState
    }

    impl FireAlarm {
        fn new<T: ToString>(gauge_name: T) -> Self {
            let name: String = gauge_name.to_string(); 
            FireAlarm { name, state: GaugeState::Disabled }
        }
    }

    impl Gauge for FireAlarm {

        fn name(&self) -> &str {
            &self.name
        }

        fn state(&self) -> &GaugeState {
            &self.state
        }

        fn set_state(&mut self, state: GaugeState) {
            self.state = state
        }

        fn is_updated(&self) -> IsUpdated {
            todo!()
        }

        fn set_is_updated(&mut self, _is_updated: IsUpdated) {
            todo!()
        }
    }

    #[test]
    fn into_bytes_and_back() {
        let gauge = FireAlarm::new("Room");

        let bytes = gauge.serialize();

        let deserialized_gauge = deserialize(&bytes).expect("Error deserializing");

        assert_eq!(deserialized_gauge.name,gauge.name);
    }
}
