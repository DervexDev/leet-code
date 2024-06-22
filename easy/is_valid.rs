impl Solution {
	pub fn is_valid(s: String) -> bool {
		let mut chars = vec![];

		for c in s.chars() {
			match c {
				'(' => chars.push(')'),
				'{' => chars.push('}'),
				'[' => chars.push(']'),
				_ => {
					if Some(c) != chars.pop() {
						return false;
					}
				}
			}
		}

		chars.is_empty()
	}
}
