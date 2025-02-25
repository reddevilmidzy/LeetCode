impl Solution {
    pub fn num_of_subarrays(mut arr: Vec<i32>) -> i32 {
        let modu = 10_i32.pow(9) + 7;
        let mut res = 0;
        let n: usize = arr.len();

        let mut odd = 0;
        let mut even = 1;
        let mut pre = 0;

        for num in arr {
            pre += num;
            if pre % 2 == 0 {
                res += odd;
                even += 1;
            } else {
                res += even;
                odd += 1;
            }
            res %= modu;
        }

        res
    }
}
