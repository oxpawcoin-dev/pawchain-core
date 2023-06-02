use crate::blockchain::{Block, Chain};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use warp::{Filter, Rejection, Reply};

#[derive(Serialize, Deserialize)]
pub struct GetBlocksRequest {
    start: usize,
    end: usize,
}

#[derive(Serialize, Deserialize)]
pub struct GetBlocksResponse {
    blocks: Vec<Block>,
}

pub struct RPC {
    chain: Arc<Mutex<Chain>>,
}

impl RPC {
    pub fn new(chain: Arc<Mutex<Chain>>) -> Self {
        RPC { chain }
    }

    pub async fn start(&self, address: impl Into<std::net::SocketAddr> + 'static) {
        let get_blocks = warp::post()
            .and(warp::path("get_blocks"))
            .and(warp::body::json())
            .and_then(move |req: GetBlocksRequest| {
                let chain = self.chain.lock().unwrap();
                let blocks = chain.get_blocks(req.start, req.end);
                let response = GetBlocksResponse { blocks };
                Ok::<_, Rejection>(warp::reply::json(&response))
            });

        let routes = get_blocks;

        warp::serve(routes).run(address).await;
    }
}
