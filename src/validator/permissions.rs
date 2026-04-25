//! Step 4 — permission-check. Verify the request's principal
//! has a capability covering the targeted slot under the
//! requested op.
//!
//! MVP: SingleOperator (SO_PEERCRED at UDS layer; trust
//! everything from this connection).
//! Post-MVP: BLS-signed capability tokens; quorum proofs.

use crate::Result;

pub fn check() -> Result<()> {
    todo!("permission-check; SingleOperator MVP; emit E0004 on unauthorised")
}
