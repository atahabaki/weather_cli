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
pub trait Forecast {
    /// Shows daily forecasts.
    fn show_daily_forecast(&self) where Self: ToDailyForecast;
    /// Shows live weather status.
    fn show_live_forecast(&self) where Self: ToLiveForecast;
}

/// A basic DailyForecast type 
/// This is an intermediate data type. Data of this is used to hold weather
/// stats, et cetera.
pub struct DailyForecast {
  pub weather: Option<String>,
  pub lowest_temp: Option<String>,
  pub highest_temp: Option<String>,
  pub lowest_humidity: Option<String>,
  pub highest_humidity: Option<String>,
  pub wind_speed: Option<String>,
  pub wind_direction: Option<String>,
}

/// Convert your type to Vector of DailyForecasts.
pub trait ToDailyForecast {
    fn to_daily_forecast(&self) -> Vec<DailyForecast>;
}

/// A basic LiveForecast type 
/// This is an intermediate data type. Data of this is used to hold weather
/// stats, et cetera.
pub struct LiveForecast {
  pub weather: Option<String>,
  pub temperature: Option<String>,
  pub humidity: Option<String>,
  pub wind_speed: Option<String>,
  pub wind_direction: Option<String>,
}

/// Convert your type to Vector of LiveForecasts.
pub trait ToLiveForecast {
    fn to_daily_forecast(&self) -> Vec<LiveForecast>;
}
