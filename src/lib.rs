//! bacdive library
//!
//! This crate exposes the BacDive CSV analysis logic (originally written
//! for the `bacdive` CLI) as a library so it can be reused by both the
//! original command line binary and the `bacdive-web` Axum web server.
//!
//! Gaurav Sablok
//! gsablok@proton.me

pub mod args;
pub mod designation;
pub mod designationlist;
pub mod idlist;
pub mod idsearch;
pub mod idwrite;
pub mod species;
pub mod specieslist;
pub mod specieswrite;
pub mod strain;
pub mod strainheader;
pub mod strainnumber;
pub mod strainwrite;
pub mod structfile;
pub mod uniqueid;
pub mod uniquespecies;
pub mod uniquestrain;
pub mod webmine;
