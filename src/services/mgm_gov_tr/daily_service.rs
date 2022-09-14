use crate::models::daily::DailyResponse;
use hyper::{Client, Uri, Request, Method, Body};
use hyper_tls::HttpsConnector;

pub struct DailyService {
    ist_no: u64
}

impl DailyService {
  pub fn new(ist_no: u64) -> Self {
    DailyService { ist_no }
  }
  pub async fn get(&self) -> Result<DailyResponse, ()> {
    let mut _path = format!("https://servis.mgm.gov.tr/web/tahminler/gunluk?istno={}", self.ist_no);
    let uri: Uri = _path.parse().unwrap();
    dbg!("{}", &uri);
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let req = Request::builder()
      .method(Method::GET)
      .uri(uri)
      .header("Origin", "https://www.mgm.gov.tr")
      .body(Body::empty())
      .expect("Request builder!");
    let res = client.request(req).await;
    let (_, body) = res.unwrap().into_parts();
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    let data = String::from_utf8(bytes.into_iter().collect()).expect("");
    let centers: DailyResponse = serde_json::from_str(data.as_str()).unwrap();
    Ok(centers)
  }
}
