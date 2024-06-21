impl Solution {
	pub fn longest_common_prefix(strs: Vec<String>) -> String {
		let chars = strs[0].chars();
		let mut cur = String::new();

		'outer: for c in chars {
			cur += &c.to_string();

			for s in strs.iter() {
				if !s.starts_with(&cur) {
					cur.pop();
					break 'outer;
				}
			}
		}

		cur
	}
}
