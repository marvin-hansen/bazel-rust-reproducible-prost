use tonic::{Request, Response, Status};

use greeting_proto::example::greeting_service_server::GreetingService;
use greeting_proto::example::{GreetingRequest, GreetingResponse};

#[derive(Copy, Clone)]
pub struct MyGreeter{}

impl MyGreeter {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl GreetingService for MyGreeter {
    async fn greet(
        &self,
        request: Request<GreetingRequest>,
    ) -> Result<Response<GreetingResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = GreetingResponse{
            greeting: format!("Hello {}!", request.into_inner().name),
            response_time: None,
        };
        Ok(Response::new(reply))
    }
}