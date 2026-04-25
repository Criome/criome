//! UDS listener — accepts signal frames from the nexus daemon.
//!
//! Skeleton-as-design.

use crate::Result;

pub struct UdsListener;

impl UdsListener {
    pub async fn bind(_socket_path: &std::path::Path) -> Result<Self> {
        todo!("bind UDS socket; SO_PEERCRED check; return listener")
    }

    pub async fn run(self) -> Result<()> {
        todo!("accept loop; spawn per-connection task; route signal frames into validator")
    }
}
