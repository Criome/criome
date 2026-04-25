//! criome daemon — binary entry point.

use criome::Result;

#[tokio::main]
async fn main() -> Result<()> {
    todo!("UDS listener (signal in) + validator pipeline + sema (redb) + lojix-schema dispatch; see ARCHITECTURE.md §3 + §4")
}
