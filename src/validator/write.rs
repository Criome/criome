//! Step 5 — write. Append `ChangeLogEntry` to the per-kind
//! redb table; update `SlotBinding`. One redb transaction
//! covers the whole TxnBatch (or single op) atomically.

use crate::Result;

pub fn apply() -> Result<()> {
    todo!("redb write transaction; per-kind ChangeLogEntry append; SlotBinding update")
}
