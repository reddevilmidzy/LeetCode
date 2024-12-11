impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let m = 100_001;
        let mut pre : Vec<i32> = vec![0; m + 1];
        for num in nums {
            pre[0.max(num - k) as usize] += 1;
            pre[m.min((num + k + 1) as usize)] -= 1;
        }
        for i in 1..=m {
            pre[i] += pre[i-1]
        }
        *pre.iter().max().unwrap()
    }
}