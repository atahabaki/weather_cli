use serde::{Deserialize, Serialize};

pub type CenterResponse = Vec<CenterStat>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CenterStat {
pub  alternatif_hadise_ist_no: Option<String>,
pub  boylam: Option<f64>,
pub  enlem: Option<f64>,
pub  gunluk_tahmin_ist_no: Option<u64>,
pub  il: Option<String>,
pub  il_plaka: Option<u64>,
pub  ilce: Option<String>,
pub  merkez_id: Option<u64>,
pub  oncelik: Option<u64>,
pub  saatlik_tahmin_ist_no: Option<u64>,
pub  sondurum_ist_no: Option<u64>,
pub  yukseklik: Option<u64>,
pub  aciklama: Option<String>,
pub  model_id: Option<u64>,
pub  gps: Option<u64>,
}
