use gfx;
use glutin;
use gfx_window_glutin;
use gfx::Device;
use gfx::traits::FactoryExt;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
	vertex Vertex {
		pos: [f32; 4] = "a_Pos",
		color: [f32; 3] = "a_Color",
	}

	pipeline pipe {
		vbuf: gfx::VertexBuffer<Vertex> = (),
		out: gfx::RenderTarget<ColorFormat> = "Target",
	}
}

pub fn main() {
	let builder = glutin::WindowBuilder::new()
		.with_title("Tea".to_string())
		.with_dimensions(800, 600)
		.with_vsync();

	let ev_loop = glutin::EventsLoop::new();

	let (window, mut device, mut factory, main_color, mut main_depth) =
		gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &ev_loop);

	let pso = factory.create_pipeline_simple(
		include_bytes!("shaders/basic.glslv"),
		include_bytes!("shaders/basic.glslf"),
		pipe::new(),
	).unwrap();

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
