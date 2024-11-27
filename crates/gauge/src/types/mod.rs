pub mod fire_alarm;
pub mod temperature_gauge;

use std::fmt::Display;

use crate::Result;

type GaugeIdentifier = Vec<u8>;
type GaugeName = String;
type SerializedGaugeBytes = Vec<u8>;

pub trait GaugeState: std::fmt::Debug 
where
    Self: Sized
{
    fn parse_state(state: SerializedState) -> Result<Self>;
}

/// ```usize```: Id length, ```Vec<u8>```: Id
type SerializedGaugeId = (usize,Vec<u8>);
/// ```usize```: Name length, ```Vec<u8>```: Name
type SerializedGaugeName = (usize,Vec<u8>);
/// ```u8```: Gauge state, 
/// ```Option<(usize,Vec<u8>)>```: Message size and message itself, if presented
type SerializedGaugeState = (u8,Option<(usize,Vec<u8>)>);

pub struct SerializedGauge {
    bytes: SerializedGaugeBytes,
    name_byte: usize,
    state_byte: usize
}

impl SerializedGauge {
    pub fn parse_id(&'static self) -> GaugeIdentifier {
        self.bytes[1..1+self.bytes[0] as usize].to_vec()
    }
}

impl From<SerializedGaugeBytes> for SerializedGauge {
    fn from(value: SerializedGaugeBytes) -> Self {
        let bytes = value.as_slice();
        let name_byte = 1+value[0] as usize; // Also name size
        let state_byte = 1+value[0]+1+value[name_byte];
        
        Self { bytes: bytes.to_vec(), name_byte, state_byte: state_byte as usize }
    }
}

pub trait Gauge
where
    Self: Sized + Display
{
    // Deserialisation
    type GaugeState: GaugeState;

    fn new(name: GaugeName, state: Self::GaugeState) -> Self;

    fn state(&self) -> &Self::GaugeState;
    fn set_state(&mut self, state: Self::GaugeState);

    fn name(&self) -> &GaugeName;
    fn set_name(&mut self, name: GaugeName);
    fn id() -> GaugeIdentifier;

    fn deserialize(gauge: SerializedGauge) -> crate::Result<Self> {
        let deserialized_gauge = DeserializedGauge::parse(gauge)?;

        Ok(Self::parse(deserialized_gauge)?)
    }

    fn parse(deserialized_gauge: DeserializedGauge) -> crate::Result<Self> {
        let name = deserialized_gauge.name();
        let state = Self::GaugeState::parse_state(deserialized_gauge.state().to_vec())?;

        Ok(Self::new( name, state))
    }

    // Serialization
    
    fn serialize_id(&self) -> SerializedGaugeId;
    fn serialize_name(&self) -> SerializedGaugeName;
    fn serialize_state(&self) -> SerializedGaugeState;

    fn serialize(&self) -> SerializedGaugeBytes {
        let id = self.serialize_id();
        let name = self.serialize_name();
        let serialized_state = self.serialize_state();
        let state_message = match serialized_state.1 {
            Some(message) => [&[message.0 as u8], message.1.as_slice()].concat(),
            None => [].into(),
        };

        let bytes = [
            &[id.0 as u8], id.1.as_slice(),
            &[name.0 as u8], name.1.as_slice(),
            &[serialized_state.0 as u8], &state_message
        ].concat();

        bytes 
    }

}

type SerializedState = Vec<u8>;

#[derive(Debug)]
pub struct DeserializedGauge
{
    id: GaugeIdentifier,
    name: String,
    state: SerializedState,
}

impl DeserializedGauge {
    pub fn name(&self) -> GaugeName {
        self.name.clone()
    }

    pub fn state(&self) -> SerializedState {
        self.state.clone()
    }

    pub fn id(&self) -> GaugeIdentifier {
        self.id.clone()
    }

    pub fn parse(gauge: SerializedGauge) -> Result<Self> {
        let id = DeserializedGauge::parse_id(&gauge);
        let name = DeserializedGauge::parse_name(&gauge)?;
        let state = DeserializedGauge::get_state(&gauge);

        Ok(Self { id, name, state })
    }

    fn parse_id(gauge: &SerializedGauge) -> GaugeIdentifier {
        let id_length = gauge.bytes[0] as usize;
        gauge.bytes[1..1+id_length].to_vec()
    }

    fn parse_name(gauge: &SerializedGauge) -> Result<GaugeName> {
        let length = gauge.bytes[gauge.name_byte] as usize;
        let slice = gauge.bytes[gauge.name_byte+1..gauge.name_byte+1+length].to_vec();

        let name =  String::from_utf8(slice)?;

        Ok(name)
    }

    fn get_state(gauge: &SerializedGauge) -> SerializedState {
        gauge.bytes[gauge.state_byte..].to_vec()
    }
}

impl Display for DeserializedGauge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_id = self.state[0];
        let mut state_message: Option<String> = None;

        if self.state.len() > 1 {
            let message = match String::from_utf8(self.state[2..].to_vec()) {
                Ok(str) => str,
                Err(_) => "<Malformed state message>".into(),
            };

            state_message = Some(message);
        };

        write!(f,"Name: {},\nId: {},\nState: {},\nState message: {}",
            self.name,
            String::from_utf8(self.id.clone()).unwrap_or("<Malformed id>".to_string()),
            state_id,
            state_message.unwrap_or("None".to_string())
        )

    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use super::{GaugeName, GaugeState, Gauge};

    const FIRE_ALARM_ID: &[u8] = "fire_alarm".as_bytes();

    #[derive(Clone,Debug,PartialEq)]
    enum FireAlarmState {
        Enabled,
        Disabled,
        Message(String)
    }

    impl GaugeState for FireAlarmState {
        fn parse_state(state: super::SerializedState) -> crate::Result<Self> {
            let state = match state[0] as u8 {
                0 => Self::Disabled,
                1 => Self::Enabled,
                2 => {
                    let message_length = state[1] as usize;
                    let message_slice = state[2..2+message_length].to_vec();
                    let message = String::from_utf8(message_slice)?;
                    Self::Message(message)
                }
                _ => Err("Fire alarm state parsing error")?
            };

            Ok(state)
        }
    }

    #[derive(Debug,Clone,PartialEq)]
    struct FireAlarm {
        name: GaugeName,
        state: FireAlarmState
    }
 
    impl Display for FireAlarm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
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
            let name = self.name.as_bytes().to_vec();
            (name.len(),name)
        }

        fn serialize_state(&self) -> super::SerializedGaugeState {
            match &self.state {
                FireAlarmState::Enabled => (1,None),
                FireAlarmState::Disabled => (0,None),
                FireAlarmState::Message(message) => {
                    let message_bytes = message.clone().into_bytes();
                    let len = message_bytes.len();
                    (2,Some((len,message_bytes)))
                },
            }
        }

        fn state(&self) -> &Self::GaugeState {
            todo!()
        }

        fn name(&self) -> &GaugeName {
            todo!()
        }

        fn id() -> super::GaugeIdentifier {
            todo!()
        }

        fn set_state(&mut self, state: Self::GaugeState) {
            todo!()
        }

        fn set_name(&mut self, name: GaugeName) {
            todo!()
        }

    }

    #[test]
    fn serialize_and_deserialize() {
        let fire_alarm = FireAlarm::new("Fire alarm".to_string(), FireAlarmState::Disabled);

        let serialized = fire_alarm.serialize();

        let deserialized = FireAlarm::deserialize(serialized.into()).expect("Error deserializing");

        println!("fist: {:?}",fire_alarm);
        println!("second: {:?}",deserialized);

        assert_eq!(fire_alarm,deserialized)

    }
}
