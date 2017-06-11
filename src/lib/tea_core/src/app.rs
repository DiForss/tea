use std::fmt::Debug;

use clap::{App, Arg};
use tea_functional::pipe::Pipe;

use interaction::cli::{CliHandler, CliIn};
use interaction::event_handler::EventHandler;

#[derive(Debug)]
pub enum Error {
	InvalidOption(String),
	EditorError(Box<Debug>),
}

pub fn main() -> Result<(), Error> {
	let matches = App::new("tea")
		.version("0.0.1")
		.author("zovt <zovt@posteo.de>")
		.about("A text editor")
		.arg(Arg::with_name("filename")
		         .short("file")
		         .index(1)
		         .multiple(true))
		.arg(Arg::with_name("interaction_mode")
		         .short("i")
		         .possible_values(&["cli"])
		         .default_value("cli"))
		.get_matches();

	let im_opt = matches.value_of("interaction_mode").unwrap();
	let (i, h) = match im_opt {
		"cli" => Ok((CliIn, CliHandler)),
		_ => Err(Error::InvalidOption(im_opt.to_owned())),
	}?;

	match ().run(i.handle_with(h).handle_with(EventHandler)) {
		Ok(_) => Ok(()),
		Err(e) => Err(Error::EditorError(Box::new(e))),
	}
}
