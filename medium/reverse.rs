impl Solution {
	pub fn reverse(mut x: i32) -> i32 {
		if x == i32::MAX || x == i32::MIN {
			return 0;
		}

		let mut digits = vec![];
		let mut rev: i32 = 0;

		while x != 0 {
			digits.push(x % 10);
			x /= 10;
		}

		for (i, x) in digits.iter().rev().enumerate() {
			if let Some(x) = x.checked_mul(10i32.pow(i as u32)) {
				if let Some(sum) = rev.checked_add(x) {
					rev = sum;
				} else {
					return 0;
				}
			} else {
				return 0;
			}
		}

		rev
	}
}
