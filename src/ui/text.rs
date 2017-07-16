use freetype::freetype::*;
use harfbuzz_sys::*;
use harfbuzz_sys::hb_direction_t::*;
use harfbuzz_sys::hb_script_t::*;

use std::ffi::CString;
use std::ptr;

pub fn basic() {
	let mut lib: FT_Library = ptr::null_mut();

	unsafe {
		assert!(FT_Init_FreeType(&mut lib as *mut FT_Library).succeeded());
	}

	let v_dpi: u32 = 72;
	let h_dpi: u32 = 72;
	let f_size: i64 = 50;

	let mut faces: [FT_Face; 1] = [ptr::null_mut()];
	let path = CString::new("src/ui/fonts/Monoid-Regular.ttf").unwrap();

	unsafe {
		assert!(
			FT_New_Face(lib, path.as_ptr(), 0, &mut faces[0] as *mut FT_Face)
				.succeeded()
		);

		assert!(
			FT_Set_Char_Size(faces[0], 0, f_size, h_dpi, v_dpi)
				.succeeded()
		);
	}

	let mut hb_fonts: [*mut hb_font_t; 1] = [ptr::null_mut()];
	unsafe {
		hb_fonts[0] = hb_ft_font_create_referenced(faces[0]);
	}

	let buf = unsafe { hb_buffer_create() };

	let lang = CString::new("en").unwrap();
	let text = CString::new("Hello, world! ffi ffff ii gh ij ft ==> =>")
		.unwrap();
	let text_ptr = text.into_raw();

	let mut glyph_count: u32 = 0;
	let glyph_info;
	let glyph_positions;

	unsafe {
		hb_buffer_set_direction(buf, HB_DIRECTION_LTR);
		hb_buffer_set_script(buf, HB_SCRIPT_LATIN);
		hb_buffer_set_language(buf, hb_language_from_string(lang.as_ptr(), -1));

		hb_buffer_add_utf8(buf, text_ptr, -1, 0, -1);
		hb_shape(hb_fonts[0], buf, ptr::null(), 0);

		glyph_info = hb_buffer_get_glyph_infos(buf, &mut glyph_count as *mut u32);
		glyph_positions = hb_buffer_get_glyph_positions(buf, &mut glyph_count as *mut u32);
	}

	println!("{}", glyph_count);

	let mut str_width_px: f32 = 0.0;
	for i in 0..glyph_count {
		str_width_px += unsafe { *glyph_positions.offset(i as isize) }.x_advance as f32 / 64.0;
	}
	str_width_px = str_width_px.ceil();

	println!("{}", str_width_px);

	unsafe {
		hb_buffer_destroy(buf);
		hb_font_destroy(hb_fonts[0]);

		assert!(FT_Done_FreeType(lib).succeeded());

		CString::from_raw(text_ptr);
	}
}
