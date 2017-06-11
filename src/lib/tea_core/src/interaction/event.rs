use tea_functional::either::Either;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum MouseType {
	Up,
	Down,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct MouseEvent {
	pub ev_type: MouseType,
	pub x: u32,
	pub y: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum KBType {
	Up,
	Down,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum KBMods {
	Shift,
	Meta,
	Alt,
	Ctrl,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct KBEvent {
	pub key: char,
	pub mods: Vec<KBMods>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum EventParseErr {
	ParseFail,
}

pub type Event = Either<MouseEvent, KBEvent>;
