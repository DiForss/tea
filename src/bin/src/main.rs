extern crate tea_core;

use tea_core::app;

fn main() {
	match app::main() {
		Ok(_) => (),
		Err(e) => println!("{:?}", e),
	};
}
