use crate::models::live::LiveResponse;
use hyper::{Client, Uri, Request, Method, Body};
use hyper_tls::HttpsConnector;

pub struct LiveService {
    merkez_id: u64
}

impl LiveService {
  pub fn new(merkez_id: u64) -> Self {
    LiveService { merkez_id }
  }
  pub async fn get(&self) -> Result<LiveResponse, ()> {
    let mut _path = format!("https://servis.mgm.gov.tr/web/sondurumlar?merkezid={}", self.merkez_id);
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
    let centers: LiveResponse = serde_json::from_str(data.as_str()).unwrap();
    Ok(centers)
  }
}
