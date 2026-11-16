//! The agent operations on a small deck (`spec/FORMAT.md` section 3): updates, structure
//! changes, then reads and whole replacements.

#[path = "ops/reads.rs"]
mod reads;
#[path = "ops/structure.rs"]
mod structure;
#[path = "ops/support.rs"]
mod support;
#[path = "ops/update.rs"]
mod update;
