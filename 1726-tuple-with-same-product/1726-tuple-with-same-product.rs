use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut dic = HashMap::new();
        let n = nums.len();
        let mut res = 0;
        
        for i in 0..n {
            for j in i+1..n {
                let tmp = nums[i] * nums[j];
                if dic.contains_key(&tmp) {
                    let val = dic.get(&tmp).unwrap();
                    res += val * 8;
                    dic.insert(tmp, val + 1);
                } else {
                    dic.insert(tmp, 1);
                }
            }
        }
        res
    }
}