use serde::{Deserialize, Serialize};

pub type LiveResponse = Vec<LiveStat>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveStat {
  pub aktuel_basinc: i64,
  pub deniz_sicaklik: i64,
  pub denize_indirgenmis_basinc: i64,
  pub gorus: i64,
  pub hadise_kodu: String,
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
