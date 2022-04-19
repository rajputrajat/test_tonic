use hello::{greeter_client::GreeterClient, HelloRequest};
use tonic::{transport::Error as TError, Request};

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
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
