use freetype::freetype::*;
use harfbuzz_sys::*;

use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::CStr;
use std::path::Path;
use std::ptr;
use std::slice::from_raw_parts;

pub fn with_new_freetype<A, F: Fn(*mut FT_Library) -> A>(f: F) -> Result<A, FT_Error> {
	let lib = ptr::null_mut();

	let err = unsafe { FT_Init_FreeType(lib) };
	if !err.succeeded() {
		return Err(err);
	};

	let out = f(lib);

	let err = unsafe { FT_Done_FreeType(*lib) };
	if !err.succeeded() {
		return Err(err);
	};

	Ok(out)
}

pub struct Glyph {
	buf: Box<[u8]>,
	width: u32,
	height: u32,
	bearing_x: i32,
	bearing_y: i32,
}

pub struct Text {
	string: String,
	direction: hb_direction_t,
	script: hb_script_t,
	language: hb_language_t,
}

pub struct Font {
	ft_face: FT_Face,
	hb_font: *mut hb_font_t,
}

impl Font {
	pub fn new(path: &Path, lib: FT_Library) -> Self {
		let path_str = path.to_string_lossy();

		let c_path = unsafe { CStr::from_bytes_with_nul_unchecked(path_str.as_bytes()) };

		let mut ft_face: FT_Face = ptr::null_mut();
		unsafe {
			FT_New_Face(lib, c_path.as_ptr(), 0, &mut ft_face);
		}

		let hb_font = unsafe { hb_ft_font_create_referenced(ft_face) };

		Font { ft_face, hb_font }
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

// Render text to a bitmap, that can be used as a texture for an
// OpenGL quad
pub fn render_text_to_bitmap(lib: FT_Library, t: Text, font: Font) -> Result<(), FT_Error> {
	let buf = unsafe { hb_buffer_create() };

	let r = render_text_to_bitmap_buf(t, font, buf);

	unsafe { hb_buffer_destroy(buf) };

	r
}

// Like render_text_to_bitmap, but you can reuse a buffer
pub fn render_text_to_bitmap_buf(text: Text,
                                 font: Font,
                                 buf: *mut hb_buffer_t)
    -> Result<(), FT_Error> {
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

	let mut glyphs: Vec<Glyph> = Vec::new();
	let total_bitmap_height = 0;
	let total_bitmap_width = 0;

	for i in 0..glyph_count {
		unsafe {
			FT_Load_Glyph(
				font.ft_face,
				(*glyph_info.offset(i as isize)).codepoint,
				FT_LOAD_DEFAULT as i32,
			)
		};

		let slot = unsafe { (*font.ft_face).glyph };
		unsafe { FT_Render_Glyph(slot, FT_Render_Mode::FT_RENDER_MODE_NORMAL) };

		let bitmap = unsafe { (*slot).bitmap };
		let size = bitmap.rows * bitmap.width;
		let bitmap_buf_slice = unsafe { from_raw_parts(bitmap.buffer, size as usize) };

		glyphs
			.push(Glyph { buf: bitmap_buf_slice.to_owned().into_boxed_slice(),
			              width: bitmap.width,
			              height: bitmap.rows,
			              bearing_x: unsafe { *slot }.bitmap_left,
			              bearing_y: unsafe { *slot }.bitmap_top, });
	}

	// FIXME: Finish this function

	Ok(())
}

fn render_glyph(lib: FT_Library) {}
