// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut nums:Vec<i32> = Vec::new();
        let mut head = head;
        while let Some(hd) = head {
            nums.push(hd.val);
            head = hd.next;
        }

        let mut res = 0;
        let n = nums.len();
        for i in 0..n {
            res += nums[n - i - 1] << (i as i32);
        }

        res
    }
}
