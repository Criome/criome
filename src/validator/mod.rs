//! Validator pipeline. Six steps — every signal request runs
//! through them in order. First failure short-circuits to a
//! `Rejected` reply with a `Diagnostic`.
//!
//! Skeleton-as-design; all step bodies are `todo!()`.

pub mod schema;
pub mod refs;
pub mod invariants;
pub mod permissions;
pub mod write;
pub mod cascade;
