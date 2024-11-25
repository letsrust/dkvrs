use anyhow::Result;
use dkvrs::{raft_srv_client::RaftSrvClient, RequestVoteRequest};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    request_vote().await?;

    Ok(())
}

async fn request_vote() -> Result<()> {
    let mut client = RaftSrvClient::connect("http://[::1]:50051").await?;

    let request = RequestVoteRequest {
        id: "1".to_string(),
    };

    let response = client.request_vote(request).await?;
    info!("Response: {response:?}");
    info!("Response: {:?}", response.into_inner());

    Ok(())
}
