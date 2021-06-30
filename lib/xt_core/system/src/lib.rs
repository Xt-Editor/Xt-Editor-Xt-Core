//! Crate for the core functions and modules of Xt.
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

#[macro_use]
extern crate slog;

extern crate xt_core_utils as utils;

pub(crate) mod server;
