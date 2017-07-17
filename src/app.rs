use ui::text;
use ui::window;

pub fn run() {
	text::with_new_freetype(|_| {});
	window::main();
}
