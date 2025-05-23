impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        // nums는 결국 nums[i] 냐 아니면 nums[i]^k냐로 정해짐
        let k = k as i64;

        let mut res = 0;
        let mut cnt = 0;
        let mut pos = i64::MAX;
        let mut neg = i64::MIN;
        for num in nums {
            let num = num as i64;
            if num >= num^k {
                res += num;
                neg = neg.max((num^k) - num);
            } else {
                res += num^k;
                pos = pos.min((num^k) - num);
                cnt += 1;
            }
        }
        if cnt%2==0 {
            return res;
        }
        (res-pos).max(res+neg)
    }
}
