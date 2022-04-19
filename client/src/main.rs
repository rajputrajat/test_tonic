use hello::{greeter_client::GreeterClient, HelloRequest};
use local_ip_address::list_afinet_netifas;
use std::{env, net::IpAddr};
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
        name: format!(
            "Hello server! I am a client and my ip-addr is '{:?}'",
            get_ip()?
        ),
    });
    let resp = client.say_hello(req).await;
    println!("response: {:?}", resp);
    Ok(())
}

#[derive(Debug)]
enum CustomError {
    TonicTransport(TError),
    LocalIp(local_ip_address::Error),
    OrgIpNotListed,
}

impl From<TError> for CustomError {
    fn from(e: TError) -> Self {
        CustomError::TonicTransport(e)
    }
}

impl From<local_ip_address::Error> for CustomError {
    fn from(e: local_ip_address::Error) -> Self {
        CustomError::LocalIp(e)
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
