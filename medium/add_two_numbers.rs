impl Solution {
	pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		Solution::sum_nodes(l1, l2, 0)
	}

	fn sum_nodes(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, num: i32) -> Option<Box<ListNode>> {
		if l1.is_none() && l2.is_none() && num == 0 {
			return None;
		}

		let l1 = l1.unwrap_or(Box::new(ListNode::new(0)));
		let l2 = l2.unwrap_or(Box::new(ListNode::new(0)));

		let sum = l1.val + l2.val + num;

		let mut cur = ListNode::new(sum % 10);
		cur.next = Self::sum_nodes(l1.next, l2.next, sum / 10);

		Some(Box::new(cur))
	}
}
