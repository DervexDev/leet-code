impl Solution {
	pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
		let mut nums = [nums1, nums2].concat();
		nums.sort();

		if nums.len() % 2 == 1 {
			nums[nums.len() / 2] as f64
		} else {
			let half = nums.len() / 2;

			(nums[half] + nums[half - 1]) as f64 / 2f64
		}
	}
}
