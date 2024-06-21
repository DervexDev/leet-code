impl Solution {
	pub fn length_of_longest_substring(s: String) -> i32 {
		let mut max = 0;
		let mut cur = String::new();

		for c in s.chars() {
			if let Some(index) = cur.find(c) {
				cur = String::from(cur.split_at(index + 1).1);
			}

			cur += &c.to_string();

			if cur.len() > max {
				max = cur.len();
			}
		}

		max as i32
	}
}
