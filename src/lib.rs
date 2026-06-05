#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]
// remove before flight
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused)]
#![allow(clippy::needless_pass_by_ref_mut)]

#[allow(nonstandard_style)]
pub mod FEMTree;
pub mod xform;
