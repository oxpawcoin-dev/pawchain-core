use crate::blockchain::Block;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

#[derive(Serialize, Deserialize)]
pub enum P2PMessage {
    NewBlock(Block),
    // Add other message types as needed
}

pub struct P2P {
    address: SocketAddr,
    peers: Vec<SocketAddr>,
    sender: mpsc::Sender<P2PMessage>,
}

impl P2P {
    pub fn new(address: SocketAddr) -> (Self, mpsc::Receiver<P2PMessage>) {
        let (sender, receiver) = mpsc::channel(100);
        (
            P2P {
                address,
                peers: vec![],
                sender,
            },
            receiver,
        )
    }

    pub async fn start(&mut self) {
        let listener = TcpListener::bind(&self.address).await.unwrap();
        loop {
            let (stream, addr) = listener.accept().await.unwrap();
            self.handle_connection(stream).await;
        }
    }

    async fn handle_connection(&mut self, mut stream: TcpStream) {
        // Implement the logic for handling incoming connections and messages
    }

    pub async fn connect_to_peer(&mut self, addr: SocketAddr) {
        match TcpStream::connect(addr).await {
            Ok(stream) => {
                self.peers.push(addr);
                self.handle_connection(stream).await;
            }
            Err(_) => {
                // Handle connection error
            }
        }
    }

    pub async fn broadcast(&mut self, message: P2PMessage) {
        // Implement the logic for broadcasting a message to connected peers
    }
}
