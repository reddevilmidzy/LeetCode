impl Solution {
    pub fn can_place_flowers(mut nums: Vec<i32>, x: i32) -> bool {
        let mut cnt = 0;
        let n = nums.len();
        if n <= 3 {
            if n == 1 {
                if nums[0] == 0 {
                    cnt += 1;
                }
            } else if n == 2 {
                if nums[0] + nums[1] == 0 {
                    cnt += 1;
                }
            } else {
                if nums[0] + nums[1] + nums[2] == 0 {
                    cnt += 2;
                } else if nums[1] == 0 && nums[0] + nums[2] == 1 {
                    cnt += 1;
                }
            }
        } else {
            if nums[0] + nums[1] == 0 {
                nums[0] = 1;
                cnt += 1;
            }
            if nums[n - 2] + nums[n - 1] == 0 {
                nums[n - 1] = 1;
                cnt += 1;
            }
            for i in 0..n - 2 {
                if nums[i] + nums[i + 1] + nums[i + 2] == 0 {
                    nums[i + 1] = 1;
                    cnt += 1;
                }
            }
        }

        cnt >= x
    }
}
