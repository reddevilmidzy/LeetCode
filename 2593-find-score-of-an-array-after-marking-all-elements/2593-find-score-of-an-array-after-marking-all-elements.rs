impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        use std::collections::HashSet;
        let n : usize = nums.len();
        let mut arr = Vec::with_capacity(n); 
        for i in 0..n {
            arr.push((nums[i], i));
        }
        arr.sort();
        let mut marked = HashSet::new();
        let mut res : i64 = 0;
        for i in 0..n {
            if !marked.contains(&arr[i].1) {
                res += arr[i].0 as i64;
                marked.insert(arr[i].1);
                marked.insert(arr[i].1+1);
                marked.insert(1.max(arr[i].1) - 1);
            }
        }
        res
    }
}