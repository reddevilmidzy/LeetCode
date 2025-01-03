impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut res = 0;

        let mut right :i64 = nums.iter().map(|x| *x as i64).sum::<i64>();

        let mut left: i64 = 0;
        if right >= 0 {
            res-=1;
        }

        for num in nums {
            let num : i64 = num as i64;
            left += num;
            right -= num;
            if left >= right {
                res += 1;
            }
        }
        res
    }
}