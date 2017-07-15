#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate harfbuzz_sys;
extern crate freetype;

mod app;
mod ui;

use app::run;

pub fn main() { run() }
