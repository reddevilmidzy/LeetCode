impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        use std::collections::HashMap;

        let n = nums.len();
        let mut res : i64 = 0;
        
        let mut low = 0;
        let mut high = 0;

        let mut di = HashMap::new();

        while high < n {
            if !di.contains_key(&nums[high]) {
                di.insert(nums[high], 0);
            }
            di.insert(nums[high], *di.get(&nums[high]).unwrap() + 1);

            while *di.keys().max().unwrap() - *di.keys().min().unwrap() > 2 {
                di.insert(nums[low], *di.get(&nums[low]).unwrap() - 1);
                if *di.get(&nums[low]).unwrap() == 0 {
                    di.remove(&nums[low]);
                }
                low += 1;
            }
            res += (high - low + 1) as i64;
            high += 1;

        }
        res 
    }
}