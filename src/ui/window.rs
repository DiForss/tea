
/*
pub fn main() {
	let builder = glutin::WindowBuilder::new()
		.with_title("Tea".to_string())
		.with_dimensions(800, 600)
		.with_vsync();

	let ev_loop = glutin::EventsLoop::new();

	let (window, mut device, mut factory, main_color, mut main_depth) =
		gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &ev_loop);

	let pso = factory
		.create_pipeline_simple(
			include_bytes!("shaders/basic.glslv"),
			include_bytes!("shaders/basic.glslf"),
			pipe::new(),
		)
		.unwrap();

	let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

	const TRI: [Vertex; 3] = [
		Vertex { pos: [-0.5, -0.5, 0.0, 1.0],
		         color: [1.0, 0.0, 0.0], },
		Vertex { pos: [0.5, -0.5, 0.0, 1.0],
		         color: [0.0, 1.0, 0.0], },
		Vertex { pos: [0.0, 0.5, 0.0, 1.0],
		         color: [0.0, 0.0, 1.0], },
	];

	let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&TRI, ());
	let mut data = pipe::Data { vbuf,
	                            out: main_color, };

	const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
	ev_loop.run_forever(|ev| {
		use glutin::WindowEvent::{Closed, KeyboardInput, Resized};

		let glutin::Event::WindowEvent { event: evnt, .. } = ev;

		match evnt {
			Closed |
			KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape), _) => ev_loop.interrupt(),
			Resized(..) => {
				gfx_window_glutin::update_views(&window, &mut data.out, &mut main_depth);
			}
			_ => (),
		}

		encoder.clear(&data.out, CLEAR_COLOR);
		encoder.draw(&slice, &pso, &data);
		encoder.flush(&mut device);
		window.swap_buffers().unwrap();
		device.cleanup();
	})
}
*/
