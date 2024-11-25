use anyhow::Result;
use dkvrs::{
    raft_srv_server::{RaftSrv, RaftSrvServer},
    RequestVoteRequest, RequestVoteResponse,
};
use tonic::{async_trait, transport::Server, Request, Response, Status};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    info!("Hello, dkvrs!");

    let addr = format!("[::1]:{}", 50051).parse()?;
    info!("Raft server listening on {addr}");

    let raft_svc = RaftService::default().into_server();
    Server::builder().add_service(raft_svc).serve(addr).await?;
    Ok(())
}

#[derive(Default)]
pub struct RaftService {}

#[async_trait]
impl RaftSrv for RaftService {
    async fn request_vote(
        &self,
        request: Request<RequestVoteRequest>,
    ) -> Result<Response<RequestVoteResponse>, Status> {
        let request_vote = request.into_inner();
        info!("RequestVote: {request_vote:?}");
        Ok(Response::new(RequestVoteResponse { vote_granted: true }))
    }
}

impl RaftService {
    pub fn into_server(self) -> RaftSrvServer<Self> {
        RaftSrvServer::new(self)
    }
}
