extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use greet::greeter_client::GreeterClient;
use greet::GreetRequest;

pub mod greet {
  tonic::include_proto!("greet");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  pretty_env_logger::init();

  let mut client = GreeterClient::connect("http://[::1]:50051").await?;

  let request = tonic::Request::new(GreetRequest { name: "Tonic".into() });

  let response = client.hello(request).await?;

  info!("RESPONSE={:?}", response);

  Ok(())
}
