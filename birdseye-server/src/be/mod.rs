use birdseye_common::rpc::be::{self, JoinRequest, JoinResponse};
use hyper::http::request;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};

pub mod tui;

pub struct BirdsEyeService {
    tokens: Arc<RwLock<Vec<String>>>,
}

#[tonic::async_trait]
impl be::birds_eye_server::BirdsEye for BirdsEyeService {
    async fn join(&self, req: Request<JoinRequest>) -> Result<Response<JoinResponse>, Status> {
        let request = req.get_ref();
        let lock = self.tokens.read().await;

        if lock.contains(&request.token) {
            Ok(Response::new(JoinResponse {}))
        } else {
            Err(Status::unauthenticated("The token provided is invalid"))
        }
    }
}
