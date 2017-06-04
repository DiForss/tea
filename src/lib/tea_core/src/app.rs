use std::fmt::Debug;

use clap::{App, Arg};
use tea_functional::stream::Stream;

use interaction::ed::{Ed, ed_handler};

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
		         .possible_values(&["ed"])
		         .default_value("ed"))
		.get_matches();

	let im_opt = matches.value_of("interaction_mode").unwrap();
	let (p, h) = match im_opt {
		"ed" => Ok((Ed, ed_handler)),
		_ => Err(Error::InvalidOption(im_opt.to_owned())),
	}?;

	match p.then(h).flush().run() {
		Ok(v) => Ok(v),
		Err(e) => Err(Error::EditorError(Box::new(e))),
	}
}
