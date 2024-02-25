use helloworld::greeter_client::GreeterClient;
use helloworld::{HelloRequest,HelloReply};
use tonic::{Request, Response};

mod helloworld{
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = Request::new(
        HelloRequest{
            name : "finally i got it".to_string()
        }
    );

    let response = client.say_hello(request).await?;

    println!("Response = {:?}",response);


    Ok(())
}