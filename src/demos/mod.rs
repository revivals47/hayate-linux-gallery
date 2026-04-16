//! Demo registry.
//!
//! Each submodule contains exactly one `Demo` impl and `inventory::submit!`s
//! itself. Adding a new demo: create a file, add a `mod foo;` line below.
//! The shell picks it up automatically via `inventory::iter`.

mod button;
mod checkbox;
mod slider;
mod text;
