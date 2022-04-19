use hello::{greeter_client::GreeterClient, HelloRequest};
use std::env;
use tonic::{transport::Error as TError, Request};

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    let addr = env::args().skip(1).collect::<Vec<String>>();
    assert_eq!(addr.len(), 2, "pass <ip-addr> and <port>");
    let mut client = GreeterClient::connect(format!("http://{}:{}", addr[0], addr[1])).await?;
    let req = Request::new(HelloRequest {
        name: "I am client. Hello server!".to_owned(),
    });
    let resp = client.say_hello(req).await;
    println!("response: {:?}", resp);
    Ok(())
}

#[derive(Debug)]
enum CustomError {
    TonicTransport(TError),
}

impl From<TError> for CustomError {
    fn from(e: TError) -> Self {
        CustomError::TonicTransport(e)
    }
}
