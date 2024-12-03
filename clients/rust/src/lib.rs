#![allow(non_local_definitions)]

mod generated;
mod hooked;

pub use {
    generated::{programs::PALADIN_LOCKUP_ID as ID, *},
    hooked::*,
};
