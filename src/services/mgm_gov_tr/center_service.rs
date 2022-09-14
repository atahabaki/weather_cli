use crate::models::mgm_gov_tr::center::CenterResponse;
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;

pub struct CenterService {
  il: String,
  ilce: Option<String>,
}

impl CenterService {
  pub fn new(il: String, ilce: Option<String>) -> Self {
    CenterService { il, ilce }
  }
  pub async fn get(&self) -> Result<CenterResponse, ()> {
    let mut _path = String::from("https://servis.mgm.gov.tr/web/merkezler?il=");
    match &self.ilce {
      Some(ilce) => {
        _path.push_str(self.il.as_str());
        _path.push_str("&ilce=");
        _path.push_str(ilce.as_str());
      }
      _ => {
        _path.push_str(self.il.as_str());
      }
    }
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
    let centers: CenterResponse = serde_json::from_str(data.as_str()).unwrap();
    Ok(centers)
  }
}
