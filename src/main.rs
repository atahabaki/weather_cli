use mgm_api::services::center_service::CenterService;
use mgm_api::services::live_service::LiveService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let center_service =
    CenterService::new("Ordu".to_owned(), Some("Ulubey".to_owned()));
  let centers = center_service.get().await;
  let centers = centers.unwrap();
  let live_service =
    LiveService::new(centers[0].merkez_id.unwrap());
  let lives = live_service.get().await;
  let lives = lives.unwrap();
  println!("{:#?}", lives);
  Ok(())
}
