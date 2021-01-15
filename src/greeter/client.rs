use tracing::info;
use tracing_subscriber;

use greet::greeter_client::GreeterClient;
use greet::GreetRequest;

pub mod greet {
  tonic::include_proto!("greet");
}

#[tokio::main(worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  tracing_subscriber::fmt::init();

  let mut client = GreeterClient::connect("http://[::1]:50051").await?;

  let request = tonic::Request::new(GreetRequest { name: "Tonic".into() });

  let response = client.hello(request).await?;

  info!("RESPONSE={:?}", response);

  Ok(())
}
