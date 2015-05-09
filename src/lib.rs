//! The 'hello' crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, hello::add_two(2));
//! ```

/// This funciton adds two to its argument.
///
/// # Examples
///
/// ```
/// use hello::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```


pub fn add_two(a: i32) -> i32 {
		 a + 2
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_adds() {
		assert_eq!(14, add_two(12));
		assert_eq!(4, add_two(2))
	}

}
