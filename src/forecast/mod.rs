//! Intermadiate data conversion, representation and et cetera...
//!
//! ## Intention
//!
//! This file is responsible for intermediate transition guidelines.
//!
//! In other words, this file is created for:
//! - To be able to add third party backends with sacrifices but without much
//! trouble or effort.
//! - Serve to the same goal, achieve the same user experience.

/// Trait to display the Forecasts.
pub trait Forecast<W> {
  /// Shows daily forecasts.
  fn show_daily_forecast(&self)
  where
    Self: ToDailyForecast<W>;
  /// Shows live weather status.
  fn show_live_forecast(&self)
  where
    Self: ToLiveForecast<W>;
}

/// Enum type for the Weather
pub enum Weather {
  /// A sunny day, or clear sky during nighttime.
  Sunny,
  /// Partially cloudy weather is when some clouds are hovering above the
  /// horizon level; however, most of the sky remains clear and there is no precipitation of any kind.
  PartiallyCloudy,
  /// Cloudy weather is when a significant amount of clouds is covering the sky
  /// (at least half the sky).
  Cloudy,
  /// Overcast weather is when the sky is completely covered by a cloud blanket.
  Overcast,
  /// Rain is the condensed moisture of the atmosphere above falling under the
  /// form of liquid droplets.
  Rain,
  /// Snow is atmospheric water that froze and fell to the ground, covering it.
  Snow,
  /// A thunderstorm is a type of weather characterized by lightning.
  Stormy,
  /// Fog is literally a cloud at ground level, which raises ambient humidity
  /// to its maximum, and considerably decreases visibility.
  Fog,
  /// Cyclones, hurricanes, and typhoons design the same phenomenon,
  /// for different parts of the world.
  Hurricane,
  /// Really hot weather.
  Hot,
  /// May freeze in an instant.
  Freezing,
  /// Unknown
  Unknown,
}

impl Weather {
    /// Transforms the Weather enum to emoji.
    fn to_emoji(&self) -> &str {
        match self {
            Weather::Sunny => "☀️",
            Weather::PartiallyCloudy => "🌤 ",
            Weather::Cloudy => "🌥 ",
            Weather::Overcast => "☁️",
            Weather::Rain => "🌧 ",
            Weather::Snow => "🌨 ",
            Weather::Stormy => "🌩 ",
            Weather::Fog => "🌁",
            Weather::Hurricane => "🌪️",
            Weather::Hot => "🌡️",
            Weather::Freezing => "🥶",
            Weather::Unknown => "🤷",
        }
    }
}

/// For strange types that needs to be converted into Weather.
pub trait ToWeather {
    fn to_weather(&self) -> Weather;
}

/// A basic DailyForecast type
/// This is an intermediate data type. Data of this is used to hold weather
/// stats, et cetera.
pub struct DailyForecast<W> {
  pub weather: W,
  pub lowest_temp: Option<String>,
  pub highest_temp: Option<String>,
  pub lowest_humidity: Option<String>,
  pub highest_humidity: Option<String>,
  pub wind_speed: Option<String>,
  pub wind_direction: Option<String>,
}

/// Convert your type to Vector of DailyForecasts.
pub trait ToDailyForecast<W> {
  fn to_daily_forecast(&self) -> Vec<DailyForecast<W>>;
}

/// A basic LiveForecast type
/// This is an intermediate data type. Data of this is used to hold weather
/// stats, et cetera.
pub struct LiveForecast<W> where W: ToWeather {
  pub weather: W,
  pub temperature: Option<String>,
  pub humidity: Option<String>,
  pub wind_speed: Option<String>,
  pub wind_direction: Option<String>,
}

/// Convert your type to Vector of LiveForecasts.
pub trait ToLiveForecast<W> {
  fn to_live_forecast(&self) -> LiveForecast<W>;
}
