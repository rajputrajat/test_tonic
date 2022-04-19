use hello::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
use local_ip_address::list_afinet_netifas;
use std::net::{AddrParseError, IpAddr, SocketAddr};
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

fn get_ip() -> Result<IpAddr, CustomError> {
    list_afinet_netifas()?
        .iter()
        .find_map(|v| {
            if v.1.to_string().contains("172.") {
                Some(v.1)
            } else {
                None
            }
        })
        .ok_or_else(|| CustomError::OrgIpNotListed)
}

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    let addr = format!("{}:50051", get_ip()?).parse::<SocketAddr>()?;
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
    LocalIp(local_ip_address::Error),
    OrgIpNotListed,
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

impl From<local_ip_address::Error> for CustomError {
    fn from(e: local_ip_address::Error) -> Self {
        CustomError::LocalIp(e)
    }
}
