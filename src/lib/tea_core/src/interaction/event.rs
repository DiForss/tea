use std::collections::HashSet;

use tea_functional::either::Either;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum State {
	Up,
	Down,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum MouseButton {
	Left,
	Right,
	Middle,
	Other(u8),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct MouseEvent {
	pub state: State,
	pub button: MouseButton,
	pub x: u32,
	pub y: u32,
}

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum KBMods {
	Shift,
	Meta,
	Alt,
	Ctrl,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KBEvent {
	pub state: State,
	pub key: char,
	pub mods: HashSet<KBMods>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum EventParseErr {
	ParseFail(String),
}

pub type Event = Either<MouseEvent, KBEvent>;
