impl Solution {
	pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
		let mut indexes = vec![];

		'outer: for (i, v) in nums.iter().enumerate() {
			for (j, w) in nums.iter().enumerate() {
				if i != j && v + w == target {
					indexes.push(i as i32);
					indexes.push(j as i32);

					break 'outer;
				}
			}
		}

		indexes
	}
}
