//! Step 1 — schema-check. Look up `kind_name` against `KindDecl`
//! records in sema; verify the request's record matches the
//! kind's field shapes.
//!
//! At first boot, validates against built-in Rust types in
//! `criome-schema` (CANON-MISSING; not yet a separate crate);
//! after `genesis.nexus` lands, validates against in-sema
//! `KindDecl` records.

use crate::Result;

pub fn check() -> Result<()> {
    todo!("schema-check; emit Diagnostic on mismatch")
}
