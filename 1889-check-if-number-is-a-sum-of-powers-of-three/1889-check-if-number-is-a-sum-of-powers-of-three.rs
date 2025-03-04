impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let nums: Vec<u32> = (0..=15).rev().map(|x| 3u32.pow(x)).collect();
        let mut x = n as u32;

        for num in nums {
            if x >= num {
                x -= num;
            }
        }

        x == 0
    }
}