//! Paladin Lockup program.

#[cfg(all(target_os = "solana", feature = "bpf-entrypoint"))]
mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;

solana_program::declare_id!("Dbf7u6x15DhjMrBMunY3XoRWdByrCCt2dbyoPrCXN6SQ");
