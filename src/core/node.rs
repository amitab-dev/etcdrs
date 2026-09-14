use std::{
    net::SocketAddr,
    sync::atomic::{AtomicU64, Ordering},
};

use crate::core::store::Store;

static NODE_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone)]
pub struct NodeId(pub u64);

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub addr: SocketAddr,
    pub state: NodeState,
    pub store: Store
}

#[derive(Debug, Clone)]
pub enum NodeState {
    Joining,
    Active,
    Leaving,
    Failed,
}

impl NodeId {
    pub fn new() -> Self {
        let id = NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
        NodeId(id)
    }
}

impl Node {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            id: NodeId::new(),
            addr,
            state: NodeState::Active,
            store: Store::new()
        }
    }
}
