impl Solution {
	pub fn is_palindrome(x: i32) -> bool {
		if x.is_negative() {
			return false;
		}

		let mut y = x;
		let mut rev = 0;

		while y != 0 {
			rev = rev * 10 + (y % 10) as u64;
			y /= 10;
		}

		x == rev as i32
	}
}
