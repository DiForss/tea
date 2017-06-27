use gfx;
use glutin;
use gfx_window_glutin;
use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub fn main() {
	let builder = glutin::WindowBuilder::new()
		.with_title("Tea".to_string())
		.with_dimensions(800, 600)
		.with_vsync();

	let ev_loop = glutin::EventsLoop::new();

	let (window, mut device, mut factory, main_color, mut main_depth) =
		gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &ev_loop);

	ev_loop.run_forever(|ev| {
		match ev {
			glutin::Event::WindowEvent { window_id: _, event: evnt } => match evnt {
				glutin::WindowEvent::Closed => ev_loop.interrupt(),
				glutin::WindowEvent::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape), _) => ev_loop.interrupt(),
				_ => (),
			}
		}
	})
}
