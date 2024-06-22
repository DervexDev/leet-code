impl Solution {
	pub fn my_atoi(s: String) -> i32 {
		let s = s.trim().to_owned();

		let mut was_sign = false;
		let mut digits: Vec<i32> = vec![];
		let mut sign = 1;

		for c in s.chars() {
			if let Ok(digit) = c.to_string().parse() {
				digits.push(digit);
				was_sign = false
			} else if c == '-' || c == '+' {
				if !digits.is_empty() || was_sign {
					break;
				}

				was_sign = true;

				if c == '-' {
					sign = -1;
				} else {
					sign = 1
				}
			} else {
				break;
			}
		}

		digits.iter().rev().enumerate().fold(0, |acc: i32, (i, &x)| {
			acc.saturating_add(x.saturating_mul(10i32.saturating_pow(i as u32)) * sign)
		})
	}
}
