use crate::{RequestVoteRequest, RequestVoteResponse};
use anyhow::Result;

pub trait RaftApi {
    fn request_vote(&self, req: RequestVoteRequest) -> Result<RequestVoteResponse>;
}
