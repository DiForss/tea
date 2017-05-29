use super::super::void::Void;

#[test]
pub fn can_use_but_not_construct() {
	let a: Option<Void> = None;
	// can't do this: a = Some(Void);
	assert_eq!(a, None);
}
