use tonic::{transport::Server, Request, Response, Status};

use tracing::info;
use tracing_subscriber;

use greet::greeter_server::{Greeter, GreeterServer};
use greet::{GreetReply, GreetRequest};

pub mod greet {
  tonic::include_proto!("greet");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
  async fn hello(&self, request: Request<GreetRequest>) -> Result<Response<GreetReply>, Status> {
    info!("Got a request: {:?}", request);

    let reply = greet::GreetReply { message: format!("Hello {}!", request.into_inner().name) };

    Ok(Response::new(reply))
  }
}

#[tokio::main(worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  tracing_subscriber::fmt::init();

  let addr = "[::1]:50051".parse()?;
  let greeter = MyGreeter::default();

  Server::builder().add_service(GreeterServer::new(greeter)).serve(addr).await?;

  Ok(())
}
