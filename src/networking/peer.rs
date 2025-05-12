#[derive(Debug, Clone)]
pub struct PeerManager;

impl PeerManager {
    pub fn new() -> Self { PeerManager }
    pub fn peer_count(&self) -> usize { 0 }
}
