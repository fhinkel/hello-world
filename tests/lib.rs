extern crate hello;

#[test]
fn it_works_integration_style() {
	assert_eq!(4, hello::add_two(2));
}