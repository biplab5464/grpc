use tonic::{transport::Server,Request,Response,Status};

use helloworld::greeter_server::{Greeter,GreeterServer};
use helloworld::{HelloRequest,HelloReply};

mod helloworld{
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box< dyn std::error::Error>>{

    let addr = "[::1]:50051".parse()?;

    let app = RpcService::default();

    Server::builder()
        .add_service(GreeterServer::new(app))
        .serve(addr)
        .await?;
    

    Ok(())

}

#[derive(Debug,Default)]
struct RpcService{}


#[tonic::async_trait]
impl Greeter for RpcService{
    async fn say_hello(&self, request : Request<HelloRequest>) -> Result<Response<HelloReply>,tonic::Status>
    {
        println!("got a request {:?}",request);
        let input = request.get_ref();
        let response = HelloReply{
            message :  format!("here i go the message {:?}", input.name)
        };
        Ok(Response::new(response))
    }
} 
