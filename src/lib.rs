//! criome — sema's engine.
//!
//! Receives [`signal`](https://github.com/LiGoldragon/signal)
//! frames over UDS from the nexus daemon. Runs every request
//! through the validator pipeline:
//!
//! 1. **schema-check** — kind well-formed against `KindDecl`?
//! 2. **ref-check** — slot-refs resolve to existing slots?
//! 3. **invariant-check** — Rule records with `is_must_hold`
//!    pass?
//! 4. **permission-check** — capability tokens / quorum?
//! 5. **write** — append to per-kind change-log; update
//!    `SlotBinding`.
//! 6. **cascade** — derived facts re-derive; subscriptions
//!    fire.
//!
//! For effects sema can't perform (compile, deploy, file
//! materialisation), criome dispatches
//! [`lojix-schema`](https://github.com/LiGoldragon/lojix-schema)
//! verbs to the [lojix](https://github.com/LiGoldragon/lojix)
//! daemon.
//!
//! Skeleton-as-design. All bodies are `todo!()`.

pub mod error;
pub mod uds;
pub mod validator;

pub use error::{Error, Result};
