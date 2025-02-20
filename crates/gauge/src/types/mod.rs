//! Модуль определяет основную архитектуру
//! для датчиков и систем умного дома.
//! 
//! # Архитектура
//!
//! От датчика требуется три свойства:
//! идентификатор, имя и состаяние.
//!
//! ## Идентификатор (Id)
//!
//! С помощью идентификатора определяется тип датчика,
//! который сообщает о том, что из себя представляет датчик:
//! пожарная сигнализация, температурный датчик и т.п.
//! 
//! Соответсвенно, зная идентификатор, можно определить,
//! как правильно прочитать информацию о его состоянии.
//!
//! ## Имя (Name)
//!
//! Как датчик представляет себя пользователю.
//!
//! ## Состояние (State)
//!
//! Состояние датчика - это `enum` который реализует [`GaugeState`].
//!
//! Состояние датчика может передавать с собйо какое-либо сообщение,
//! для этого необходимо определить логику для сериализиции [`Gauge::serialize_state`] и десериализации
//! состояния [`GaugeState::parse_state`]
//!
//! ## Примеры 
//! 
//! [`fire_alarm`]
//! [`temperature_gauge`]
//! 


pub mod fire_alarm;
pub mod temperature_gauge;

const END_OF_TRANSMISSION: char = '\x04';

use std::{fmt::Display, collections::HashMap};

use crate::Result;

type GaugeIdentifier = Vec<u8>;
type GaugeName = String;

pub trait GaugeState: std::fmt::Debug 
{
    fn parse_state(state: SerializedState) -> Result<Self> where Self: Sized;
}

type SerializedGaugeBytes = Vec<u8>;

/// Сериализованный счетчик
pub struct SerializedGauge(SerializedGaugeBytes);

/// Читает сериализованный счетчик в виде TCP сообщения,
/// которое оканчивается символом [`END_OF_TRANSMISSION`]
impl From<SerializedGaugeBytes> for SerializedGauge {
    fn from(value: SerializedGaugeBytes) -> Self {
        let bytes = value.split(|byte| 
            { *byte as char == END_OF_TRANSMISSION }
        )
            .collect::<Vec<&[u8]>>()[0];

        Self(bytes.to_vec())
    }
}

type SerializedGaugeId = Vec<u8>;
type SerializedGaugeName = Vec<u8>;
/// ```u8```: Статус счетчика, 
/// ```Option<Vec<u8>>```: Сообщение от счетчика, если имеется
type SerializedGaugeState = (u8,Option<Vec<u8>>);

pub trait Gauge
where
    Self: Display + std::fmt::Debug
{
    // Deserialisation
    type GaugeState: GaugeState;

    fn new(name: GaugeName, state: Self::GaugeState) -> Self where Self: Sized;

    fn state(&self) -> &Self::GaugeState;
    fn set_state(&mut self, state: Self::GaugeState);

    fn name(&self) -> &GaugeName;
    fn set_name(&mut self, name: GaugeName);

    fn id() -> GaugeIdentifier where Self: Sized;

    fn deserialize(gauge: SerializedGauge) -> crate::Result<Self> where Self: Sized {
        let deserialized_gauge = DeserializedGauge::parse(gauge);

        Ok(Self::parse(deserialized_gauge)?)
    }

    fn parse(deserialized_gauge: DeserializedGauge) -> crate::Result<Self> where Self: Sized {
        let name = deserialized_gauge.try_name()?;
        let state = Self::GaugeState::parse_state(deserialized_gauge.try_state()?)?;

        Ok(Self::new( name, state))
    }

    // Serialization

    fn serialize_id(&self) -> SerializedGaugeId;
    fn serialize_name(&self) -> SerializedGaugeName;
    fn serialize_state(&self) -> SerializedGaugeState;

    /// Сериализует счетчик в вид `Ключ:Значение;Ключ:Значение`,
    /// Для дальнейщей десериализации в Хэш Таблиц (см. [`DeserializedGauge`]).
    fn serialize(&self) -> SerializedGaugeBytes where Self: Sized {
        let id = Self::id();
        let name = self.name().as_bytes().to_vec();

        let raw_state = self.serialize_state();
        let state = [
            [raw_state.0].to_vec(),
            match raw_state.1 {
                Some(message) => message,
                None => vec![],
            }
        ]
            .concat();

        let mut map: HashMap<&str, Vec<u8>> = HashMap::new();
        let mut serialized_slice = Vec::new();

        map.insert("id", id);
        map.insert("name", name);
        map.insert("state", state);

        println!("{:?}",&map);

        for (key, value) in map {
            serialized_slice.push([
                key, 
                &unsafe { String::from_utf8_unchecked(value) }
            ].join(":"))
        }

        let mut serialized_string = serialized_slice.join(";");
        serialized_string.push(END_OF_TRANSMISSION);
       
        println!("{:?}",serialized_string);

        serialized_string.into_bytes()
    }

}

type SerializedState = Vec<u8>;

/// Представляет собой десериализованный счетчик в виде
/// хэш-таблицы с ключем `String` и значением `Vec<u8>`
#[derive(Debug)]
pub struct DeserializedGauge(HashMap<String,Vec<u8>>);

impl DeserializedGauge {
    /// Читает имя из байтов
    pub fn try_name(&self) -> Result<GaugeName>{
        match self.0.get("name") {
            Some(val) => Ok(String::from_utf8(val.clone())?),
            None => Err(DeserializedGaugeError::NameNotFound)?,
        }
    }

    /// Читает состояние из байтов
    pub fn try_state(&self) -> Result<SerializedState>{
        match self.0.get("state") {
            Some(val) => Ok(val.clone()),
            None => Err(DeserializedGaugeError::StateNotFound)?,
        }
    }

    /// Читает идентификатор из байтов
    pub fn try_id(&self) -> Result<GaugeIdentifier>{
        match self.0.get("id") {
            Some(val) => Ok(val.clone()),
            None => Err(DeserializedGaugeError::IdNotFound)?,
        }
    }

    pub fn parse(gauge: SerializedGauge) -> Self {
        let bytes_as_string = unsafe { String::from_utf8_unchecked(gauge.0) };

        println!("{:?}",bytes_as_string);
        
        let map: HashMap<String, Vec<u8>> = HashMap::from(
            bytes_as_string.split(';')
                .map(|val| {
                    let key_value: Vec<&str> = val.split(':').collect();
                    (key_value[0].to_string(),key_value[1].as_bytes().to_vec())
                })
                .collect::<HashMap<String,Vec<u8>>>()
        );

        println!("{:?}",map);

        Self(map)
    }

}

impl Display for DeserializedGauge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (state,state_message): (String,String) = match self.try_state() {
            Ok(state) => {
                let state_id = format!("{}",state[0]);
                let state_message: String;

                if state.len() > 1 {
                    state_message = match String::from_utf8(state[1..].to_vec()) {
                        Ok(str) => str,
                        Err(_) => "<Malformed state message>".into(),
                    };
                } else {
                    state_message = "None".to_string();
                };

                (state_id,state_message)
            },
            Err(_) => {
                ("<Malformed state>".into(),"<Malformed state message>".into())
            },
        };
        let id_bytes = self.try_id().unwrap_or("<Malformed id>".into());

        write!(f,"Name: {},\nId: {},\nState: {},\nState message: {}",
            self.try_name().unwrap_or("<Malformed name>".into()),
            String::from_utf8(id_bytes).unwrap_or("<Malformed id>".to_string()),
            state,
            state_message
        )

    }
}

#[derive(Debug)]
enum DeserializedGaugeError {
    NameNotFound,
    IdNotFound,
    StateNotFound
}

/// Выводит Debug
impl Display for DeserializedGaugeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self)
    }
}

impl std::error::Error for DeserializedGaugeError {}
