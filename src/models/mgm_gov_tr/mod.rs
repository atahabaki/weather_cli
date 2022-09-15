use crate::forecast::{Weather, ToWeather};

pub mod center;
pub mod daily;
pub mod live;

pub type Hadise = String;

impl ToWeather for Hadise {
    fn to_weather(&self) -> Weather {
        match self.as_str() {
            "PB" => Weather::PartiallyCloudy,
            "A" => Weather::Sunny,
            "AB" => Weather::Cloudy,
            "GSY" => Weather::Stormy,
            "SCK" => Weather::Hot,
            "SY" => Weather::Rain,
            _ => Weather::Unknown
        }
    }
}
