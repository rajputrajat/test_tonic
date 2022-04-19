use hello::{greeter_server::Greeter, HelloReply, HelloRequest};
use tonic::{Request, Response, Status};

pub mod hello {
    tonic::include_proto!("hello");
}

pub struct MyGreeeterServer;

#[tonic::async_trait]
impl Greeter for MyGreeeterServer {
    async fn say_hello(&self, req: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        println!("got a req: '{:?}'", req);
        let reply = HelloReply {
            message: "Hi there".to_owned(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() {}
