use gtk;
use gtk::{Button, Window, WindowType};
use gtk::prelude::*;

pub fn run() {
	if gtk::init().is_err() {
		println!("Failed to init gtk");
		return;
	}

	let window = Window::new(WindowType::Toplevel);
	window.set_title("Tea");
	window.set_default_size(800, 600);
	let button = Button::new_with_label("Hello");
	window.add(&button);
	window.show_all();

	window.connect_delete_event(|_, _| {
		                            gtk::main_quit();
		                            Inhibit(false)
		                           });

	button.connect_clicked(|_| {
		                       println!("Clicked!");
		                      });

	gtk::main();
}
