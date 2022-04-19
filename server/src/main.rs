use std::net::{AddrParseError, SocketAddr};

use hello::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
use tonic::{
    transport::{Error as TError, Server},
    Request, Response, Status,
};

pub mod hello {
    tonic::include_proto!("hello");
}

pub struct MyGreeeterServer;

#[tonic::async_trait]
impl Greeter for MyGreeeterServer {
    async fn say_hello(&self, req: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        println!("got a req: '{:?}'", req);
        let reply = HelloReply {
            message: "Hi there. I am server".to_owned(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    let addr = "[::]:50051".parse::<SocketAddr>()?;
    let greeter = MyGreeeterServer;
    println!("listening on '{}'", addr);
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}

#[derive(Debug)]
enum CustomError {
    TonicTransport(TError),
    SockerParser(AddrParseError),
}

impl From<TError> for CustomError {
    fn from(e: TError) -> Self {
        CustomError::TonicTransport(e)
    }
}

impl From<AddrParseError> for CustomError {
    fn from(e: AddrParseError) -> Self {
        CustomError::SockerParser(e)
    }
}
