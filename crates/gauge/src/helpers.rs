//! Вспомогательные функции
use json_minimal::Json;

use crate::types::{DeserializedGauge, fire_alarm::{FireAlarm, FIRE_ALARM_ID}, Gauge, temperature_gauge::{TemperatureGauge, TEMPERATURE_GAUGE_ID}};

pub enum Gauges {
    FireAlarm(FireAlarm),
    TemperatureGauge(TemperatureGauge),
    Unknown(DeserializedGauge)
}

pub fn read_gauge_by_id(gauge: DeserializedGauge) -> super::Result<Gauges> {
    let gauge = match gauge.try_id()?.as_slice() {
        FIRE_ALARM_ID => Gauges::FireAlarm(FireAlarm::parse(gauge)?),
        TEMPERATURE_GAUGE_ID => Gauges::TemperatureGauge(TemperatureGauge::parse(gauge)?),
        _ => Gauges::Unknown(gauge)
    };

    Ok(gauge)
}

const NUMBER_EXPECTED: &str = "expected to be number";
const ARRAY_EXPECTED: &str = "expected to be array";
const MALFORMED_JSON: &str = "malformed json";
const NOT_AN_OBJECT: &str = "json is not an object";

pub fn json_check_object<'a>(value: &'a Json) -> crate::Result<&'a Json> {
    match value {
        Json::OBJECT { name: _, value } => Ok(value.unbox()),
        _ => Err(NOT_AN_OBJECT)?
    }
}

pub fn json_check_array<T: for<'a> TryFrom<&'a Json>>(value: &Json, field: &str, on_absence: crate::Result<Vec<T>>) -> crate::Result<Vec<T>> {
    let array_json = match value.get(field) {
        Some(json) => match json {
            Json::OBJECT { name: _, value } => value.unbox(),
            _ => Err(MALFORMED_JSON)?
        }
        None => return on_absence,
    };

    let array = match array_json {
        Json::ARRAY(arr) => arr,
        _ => Err([field,ARRAY_EXPECTED].join(" "))?
    };

    let mut result: Vec<T> = vec![];

    for val in array {
        match T::try_from(val) {
            Ok(data) => result.push(data),
            Err(_e) => Err("Error parsing value in array")?,
        };
    }

    Ok(result)
}

pub fn json_check_number<T: From<f64>>(value: &Json, field: &str, on_absence: crate::Result<T>) -> crate::Result<T> {
    match value.get(field) {
        Some(v) => match v {
            Json::OBJECT { name: _, value } => {
                match value.unbox() {
                    Json::NUMBER(number) => Ok(T::from(*number)),
                    _ => Err([field,NUMBER_EXPECTED].join(" "))?
                }
            },
            _ => Err(MALFORMED_JSON)?
        },
        None => on_absence,
    }
}

