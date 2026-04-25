//! Step 2 — ref-check. Every slot-ref in the request must
//! resolve to an existing slot in sema (or to a slot bound
//! earlier in the same atomic batch — but forward-refs within
//! one TxnBatch are NOT resolved per
//! [criome/ARCHITECTURE.md](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md)).

use crate::Result;

pub fn check() -> Result<()> {
    todo!("ref-check; emit Diagnostic on dangling slot")
}
