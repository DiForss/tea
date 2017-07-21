use freetype::freetype::*;

use gfx::{Factory, Resources};
use gfx::handle::ShaderResourceView;

use harfbuzz_sys::*;

use std;
use std::borrow::Cow;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr;
use std::slice::from_raw_parts;

use ui::graphics::texture_from_data;

pub fn with_new_freetype<A, F: Fn(FT_Library) -> A>(f: F) -> Result<A, FT_Error> {
	let mut lib: FT_Library = ptr::null_mut();

	let err = unsafe { FT_Init_FreeType(&mut lib) };
	if !err.succeeded() {
		return Err(err);
	};

	let out = f(lib);

	let err = unsafe { FT_Done_FreeType(lib) };
	if !err.succeeded() {
		return Err(err);
	};

	Ok(out)
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct GlyphKey {
	pub codepoint: u32,
	// The glyph's size in px
	pub size: u16,
}

impl GlyphKey {
	pub fn new(codepoint: u32, size: u16) -> Self { GlyphKey { codepoint, size } }
}

#[derive(Debug)]
pub struct Glyph<R>
where
	R: Resources, {
	// Handle to the glyph texture on the GPU
	pub texture: ShaderResourceView<R, [f32; 4]>,
	pub width: u16,
	pub height: u16,
	pub bearing_x: i32,
	pub bearing_y: i32,
	pub advance_x: i32,
	pub advance_y: i32,
}

pub type GlyphLibrary<R> = HashMap<GlyphKey, Glyph<R>>;

pub struct Text {
	pub string: String,
	pub direction: hb_direction_t,
	pub script: hb_script_t,
	pub language: hb_language_t,
}

pub struct Font {
	ft_face: FT_Face,
	hb_font: *mut hb_font_t,
}

impl Font {
	pub fn new(lib: FT_Library, path: &Path) -> Self {
		let path_str = CString::new(path.to_str().unwrap()).unwrap();

		let mut ft_face: FT_Face = ptr::null_mut();
		unsafe {
			FT_New_Face(lib, path_str.as_ptr(), 0, &mut ft_face);
		}

		let hb_font = unsafe { hb_ft_font_create_referenced(ft_face) };

		Font { ft_face, hb_font }
	}

	pub fn set_pixel_size(&mut self, px_size: u32) {
		unsafe {
			FT_Set_Pixel_Sizes(self.ft_face, 0, px_size);
		}
	}

	pub fn get_name(&self) -> Cow<str> {
		unsafe { CStr::from_ptr((*self.ft_face).family_name) }
			.to_string_lossy()
	}
}

impl Drop for Font {
	fn drop(&mut self) {
		unsafe { hb_font_destroy(self.hb_font) };
		unsafe { FT_Done_Face(self.ft_face) };
	}
}

pub type FontLibrary = HashMap<String, Font>;

// Render Text to a vector of Glyphs. Any newly created glyphs will be
// created as textures
pub fn render_text_to_glyphs<'a, R, F>(t: Text,
                                       font: Font,
                                       library: &'a mut GlyphLibrary<R>,
                                       factory: &mut F)
    -> Result<Vec<&'a Glyph<R>>, FT_Error>
where
	R: Resources,
	F: Factory<R>, {
	let buf = unsafe { hb_buffer_create() };

	let r = render_text_to_glyphs_buf(t, font, buf, library, factory);

	unsafe { hb_buffer_destroy(buf) };

	r
}

// Like render_text_to_bitmap, but you can reuse a buffer
pub fn render_text_to_glyphs_buf<'a, R, F>(text: Text,
                                           font: Font,
                                           buf: *mut hb_buffer_t,
                                           library: &'a mut GlyphLibrary<R>,
                                           factory: &mut F)
    -> Result<Vec<&'a Glyph<R>>, FT_Error>
where
	R: Resources,
	F: Factory<R>, {
	unsafe {
		hb_buffer_reset(buf);
		hb_buffer_set_direction(buf, text.direction);
		hb_buffer_set_script(buf, text.script);
		hb_buffer_set_language(buf, text.language);
	}

	let len = text.string.len();

	unsafe {
		hb_buffer_add_utf8(
			buf,
			text.string.as_ptr() as *const i8,
			len as i32,
			0,
			len as i32,
		)
	};

	unsafe { hb_shape(font.hb_font, buf, ptr::null(), 0) }

	let mut glyph_count = 0;
	let glyph_info;
	let glyph_pos;

	unsafe {
		glyph_info = hb_buffer_get_glyph_infos(buf, &mut glyph_count);
		glyph_pos = hb_buffer_get_glyph_positions(buf, &mut glyph_count);
	}

	let mut glyphs: Vec<&Glyph<R>> = Vec::with_capacity(glyph_count as usize);

	for i in 0..glyph_count {
		let codepoint = unsafe { (*glyph_info.offset(i as isize)).codepoint };
		let font_size = unsafe { *(*font.ft_face).size }.metrics.y_ppem;

		let key = GlyphKey::new(codepoint, font_size);

		library.get(&key).map_or_else(
			|| {
				unsafe { FT_Load_Glyph(font.ft_face, codepoint, FT_LOAD_DEFAULT as i32) };

				let slot = unsafe { (*font.ft_face).glyph };
				unsafe { FT_Render_Glyph(slot, FT_Render_Mode::FT_RENDER_MODE_NORMAL) };

				let bitmap = unsafe { (*slot).bitmap };
				let size = bitmap.rows * bitmap.width;
				let bitmap_buf_slice = unsafe { from_raw_parts(bitmap.buffer, size as usize) };

				let pos = unsafe { (*glyph_pos.offset(i as isize)) };

				let glyph = Glyph { texture: texture_from_data(
					factory,
					bitmap_buf_slice,
					bitmap.width as u16,
					bitmap.rows as u16,
				),
				                    width: bitmap.width as u16,
				                    height: bitmap.rows as u16,
				                    bearing_x: unsafe { *slot }.bitmap_left,
				                    bearing_y: unsafe { *slot }.bitmap_top,
				                    advance_x: pos.x_advance,
				                    advance_y: pos.y_advance, };

				library.insert(key, glyph);
				glyphs.push(library.get(&key).unwrap());
			},
			|g| glyphs.push(g),
		);
	}

	Ok(glyphs)
}
