

use harfbuzz_sys::*;
use std::ffi::CString;

use std::path::Path;
use ui::text;
use ui::window;

pub fn run() {
	text::with_new_freetype(|lib| {
		let mut font = text::Font::new(lib, Path::new("src/ui/fonts/Monoid-Regular.ttf"));
		font.set_pixel_size(16);

		let en = CString::new("en").unwrap();

		let text = text::Text { string: "This is a test string ffi =>".to_string(),
		                        direction: HB_DIRECTION_LTR,
		                        script: HB_SCRIPT_LATIN,
		                        language: unsafe {
			                        hb_language_from_string(en.as_ptr(), en.as_bytes().len() as i32)
			                       }, };

		text::render_text_to_glyphs(text, font);

		()
	}).unwrap();
	// 	window::main();
}
