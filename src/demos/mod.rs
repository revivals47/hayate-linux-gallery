//! Demo registry.
//!
//! Each submodule contains exactly one `Demo` impl and `inventory::submit!`s
//! itself. Adding a new demo: create a file, add a `mod foo;` line below.
//! The shell picks it up automatically via `inventory::iter`.

mod breadcrumb;
mod button;
mod canvas;
mod checkbox;
mod color_picker;
mod combo_box;
mod dropdown;
mod grid;
mod menu_bar;
mod progress_bar;
mod radio;
mod slider;
mod spin_button;
mod split_view;
mod switch;
mod text;
mod text_input;
mod toolbar;
mod tooltip;
