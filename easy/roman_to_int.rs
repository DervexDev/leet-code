impl Solution {
	pub fn roman_to_int(s: String) -> i32 {
		let mut last_v = 0;
		let mut sum = 0;

		for c in s.chars() {
			let v = Self::convert(c);

			if last_v < v {
				sum -= last_v * 2;
			}

			sum += v;
			last_v = v;
		}

		sum
	}

	#[inline]
	fn convert(c: char) -> i32 {
		match c {
			'I' => 1,
			'V' => 5,
			'X' => 10,
			'L' => 50,
			'C' => 100,
			'D' => 500,
			'M' => 1000,
			_ => 0,
		}
	}
}
