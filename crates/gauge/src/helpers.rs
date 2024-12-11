//! Вспомогательные функции
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
