//! Step 3 — invariant-check. Run every Rule record with
//! `is_must_hold = true` against the post-write sema snapshot;
//! reject if any rule's body fails to match the head.

use crate::Result;

pub fn check() -> Result<()> {
    todo!("invariant-check; emit Diagnostic with the failing rule's slot")
}
