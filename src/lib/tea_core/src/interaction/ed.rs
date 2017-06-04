use std::io::{stdout, Error as IOError, Write};

use tea_functional::stream::{Stream, Poll, StdinLn, forever};

use interaction::interface::Interface;

/* 
This interaction mode is similar to the text editor "ed"
in that it reads commands from the terminal.

This probably won't be a full featured tea front-end, but
who knows? :)
 */

pub struct Ed;

impl Stream for Ed {
	type Value = String;
	type Final = Result<(), IOError>;
	
	fn poll(&mut self) -> Poll<Self::Value, Self::Final> {
		print!("tea> ");
		match stdout().flush() {
			Ok(_) => (),
			Err(e) => return Poll::Done(Err(e))
		}

		forever(10).take(2).discard_final().map_final(|_| Ok(())).map(|i| i.to_string()).poll()
	}
}

impl Interface<String, IOError> for Ed{}

pub fn ed_handler(v: String) -> Poll<(), Result<(), IOError>> {
	println!("{}", v);

	Poll::Some(())
}
