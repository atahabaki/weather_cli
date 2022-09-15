use serde::{Deserialize, Serialize};

use crate::forecast::{ToLiveForecast, LiveForecast};

use super::Hadise;

pub type LiveResponse = Vec<LiveStat>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveStat {
  pub aktuel_basinc: i64,
  pub deniz_sicaklik: i64,
  pub denize_indirgenmis_basinc: i64,
  pub gorus: i64,
  pub hadise_kodu: Hadise,
  pub ist_no: i64,
  pub kapalilik: i64,
  pub kar_yukseklik: i64,
  pub nem: i64,
  pub rasat_metar: String,
  pub rasat_sinoptik: String,
  pub rasat_taf: String,
  pub ruzgar_hiz: f64,
  pub ruzgar_yon: i64,
  pub sicaklik: f64,
  pub veri_zamani: String,
  #[serde(rename = "yagis00Now")]
  pub yagis00now: f64,
  #[serde(rename = "yagis10Dk")]
  pub yagis10dk: i64,
  #[serde(rename = "yagis12Saat")]
  pub yagis12saat: i64,
  #[serde(rename = "yagis1Saat")]
  pub yagis1saat: i64,
  #[serde(rename = "yagis24Saat")]
  pub yagis24saat: f64,
  #[serde(rename = "yagis6Saat")]
  pub yagis6saat: i64,
  pub deniz_veri_zamani: String,
}

impl ToLiveForecast<Hadise> for LiveStat {
  fn to_live_forecast(&self) -> LiveForecast<Hadise> {
    LiveForecast {
      weather: self.hadise_kodu.clone(),
      temperature: self.sicaklik.to_string().into(),
      humidity: self.nem.to_string().into(),
      wind_speed: self.ruzgar_hiz.to_string().into(),
      wind_direction: self.ruzgar_yon.to_string().into(),
    }
  }
}
