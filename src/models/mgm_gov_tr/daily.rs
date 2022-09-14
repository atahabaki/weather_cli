use serde::{Deserialize, Serialize};

use crate::forecast::{DailyForecast, ToDailyForecast};

pub type DailyResponse = Vec<DailyStat>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyStat {
  pub en_dusuk_gun1: i64,
  pub en_dusuk_gun2: i64,
  pub en_dusuk_gun3: i64,
  pub en_dusuk_gun4: i64,
  pub en_dusuk_gun5: i64,
  pub en_dusuk_nem_gun1: i64,
  pub en_dusuk_nem_gun2: i64,
  pub en_dusuk_nem_gun3: i64,
  pub en_dusuk_nem_gun4: i64,
  pub en_dusuk_nem_gun5: i64,
  pub en_yuksek_gun1: i64,
  pub en_yuksek_gun2: i64,
  pub en_yuksek_gun3: i64,
  pub en_yuksek_gun4: i64,
  pub en_yuksek_gun5: i64,
  pub en_yuksek_nem_gun1: i64,
  pub en_yuksek_nem_gun2: i64,
  pub en_yuksek_nem_gun3: i64,
  pub en_yuksek_nem_gun4: i64,
  pub en_yuksek_nem_gun5: i64,
  pub hadise_gun1: String,
  pub hadise_gun2: String,
  pub hadise_gun3: String,
  pub hadise_gun4: String,
  pub hadise_gun5: String,
  pub ist_no: i64,
  pub ruzgar_hiz_gun1: i64,
  pub ruzgar_hiz_gun2: i64,
  pub ruzgar_hiz_gun3: i64,
  pub ruzgar_hiz_gun4: i64,
  pub ruzgar_hiz_gun5: i64,
  pub ruzgar_yon_gun1: i64,
  pub ruzgar_yon_gun2: i64,
  pub ruzgar_yon_gun3: i64,
  pub ruzgar_yon_gun4: i64,
  pub ruzgar_yon_gun5: i64,
  pub tarih_gun1: String,
  pub tarih_gun2: String,
  pub tarih_gun3: String,
  pub tarih_gun4: String,
  pub tarih_gun5: String,
}

impl ToDailyForecast for DailyStat {
    fn to_daily_forecast(&self) -> Vec<DailyForecast> {
        vec![
            DailyForecast {
                weather: self.hadise_gun1.into(),
                lowest_temp: self.en_dusuk_gun1.to_string().into(),
                highest_temp: self.en_yuksek_gun1.to_string().into(),
                lowest_humidity: self.en_dusuk_nem_gun1.to_string().into(),
                highest_humidity: self.en_yuksek_nem_gun1.to_string().into(),
                wind_speed: self.ruzgar_hiz_gun1.to_string().into(),
                wind_direction: self.ruzgar_yon_gun1.to_string().into()
            },
            DailyForecast {
                weather: self.hadise_gun2.into(),
                lowest_temp: self.en_dusuk_gun2.to_string().into(),
                highest_temp: self.en_yuksek_gun2.to_string().into(),
                lowest_humidity: self.en_dusuk_nem_gun2.to_string().into(),
                highest_humidity: self.en_yuksek_nem_gun2.to_string().into(),
                wind_speed: self.ruzgar_hiz_gun2.to_string().into(),
                wind_direction: self.ruzgar_yon_gun2.to_string().into()
            },
            DailyForecast {
                weather: self.hadise_gun3.into(),
                lowest_temp: self.en_dusuk_gun3.to_string().into(),
                highest_temp: self.en_yuksek_gun3.to_string().into(),
                lowest_humidity: self.en_dusuk_nem_gun3.to_string().into(),
                highest_humidity: self.en_yuksek_nem_gun3.to_string().into(),
                wind_speed: self.ruzgar_hiz_gun3.to_string().into(),
                wind_direction: self.ruzgar_yon_gun3.to_string().into()
            },
            DailyForecast {
                weather: self.hadise_gun4.into(),
                lowest_temp: self.en_dusuk_gun4.to_string().into(),
                highest_temp: self.en_yuksek_gun4.to_string().into(),
                lowest_humidity: self.en_dusuk_nem_gun4.to_string().into(),
                highest_humidity: self.en_yuksek_nem_gun4.to_string().into(),
                wind_speed: self.ruzgar_hiz_gun4.to_string().into(),
                wind_direction: self.ruzgar_yon_gun4.to_string().into()
            },
            DailyForecast {
                weather: self.hadise_gun5.into(),
                lowest_temp: self.en_dusuk_gun5.to_string().into(),
                highest_temp: self.en_yuksek_gun5.to_string().into(),
                lowest_humidity: self.en_dusuk_nem_gun5.to_string().into(),
                highest_humidity: self.en_yuksek_nem_gun5.to_string().into(),
                wind_speed: self.ruzgar_hiz_gun5.to_string().into(),
                wind_direction: self.ruzgar_yon_gun5.to_string().into()
            },
        ]
    }
}
