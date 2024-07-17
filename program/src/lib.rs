//! Paladin Lockup program.

#[cfg(all(target_os = "solana", feature = "bpf-entrypoint"))]
mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

solana_program::declare_id!("AAJBV83AXRjN5sEF8gWjyo6mVxLv822Mg1jYp1PZ8MCo");
