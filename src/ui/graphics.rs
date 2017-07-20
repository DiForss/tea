use gfx;
use gfx::Device;
use gfx::traits::FactoryExt;
use gfx_window_glutin;
use glutin;

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

pub struct Position2D {
	x: f32,
	y: f32,
}

// All graphical primitives will be oriented on their top-left corner
// for now

pub struct Quad<V> {
	// Vertices will always be [0.0, 1.0], with the size always being
	// such that either width or height == 1.
	// Vertices are ordered: TL, BL, BR, TR
	vertices: [V; 4],

	// How much we need to scale the Quad by to get the desired width
	// and height
	scale: f32,
}

impl<V> Quad<V>
where
	V: From<(f32, f32)> + Into<(f32, f32)> + Copy, {
	pub fn new(w: f32, h: f32) -> Self {
		let (adj_w, adj_h, scale) = if w > h {
			(1.0, h / w, w)
		} else {
			(w / h, 1.0, w)
		};

		let vertices = [
			V::from((0.0, 0.0)),
			V::from((0.0, adj_h)),
			V::from((adj_w, adj_h)),
			V::from((adj_w, 0.0)),
		];

		Quad { vertices, scale }
	}

	pub fn width(&self) -> f32 { self.vertices[3].into().0 * self.scale }
	pub fn height(&self) -> f32 { self.vertices[1].into().1 * self.scale }

	// Return an EBO laid out like:
	// T1: TL BL TR
	// T2: BL TR BR
	pub fn ebo(&self) -> [i8; 6] { [0, 1, 3, 1, 2, 3] }
}
