use super::super::identity::identity;

#[test]
fn test_identity() {
	assert_eq!(8, identity(8));
	assert_eq!("hello", identity(identity("hello")));
}
