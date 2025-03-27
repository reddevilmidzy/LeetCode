use std::collections::HashMap;

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut cnt: HashMap<i32, i32> = HashMap::new();

        for &num in &nums {
            *cnt.entry(num).or_insert(0) += 1;
        }
        let mut x = 0;
        let mut x_val = 0;
        for (key,val) in cnt.iter() {
            if *val > (n/2) as i32 {
                x = *key;
                x_val = *val;
            }
        }

        let mut x_cnt = 0;
        for i in 0..n {
            if nums[i] == x {
                x_cnt += 1;
            }
            if x_cnt*2 > (i+1) as i32 && (x_val-x_cnt)*2 > (n-i-1) as i32 {
                return i as i32;
            }
        }

        -1
    }
}
