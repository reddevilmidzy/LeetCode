impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n : usize = nums.len();
        let mut arr : Vec<i32> = vec![0; n];

        for i in 0..n-1 {
            if nums[i]%2 == nums[i+1]%2 {
                arr[i+1] = 1;
            }
            arr[i+1] += arr[i];
        }
        let mut res : Vec<bool> = Vec::with_capacity(queries.len());

        for query in queries {
            if arr[query[1] as usize] - arr[query[0] as usize] != 0 {
                res.push(false);
            } else {
                res.push(true);
            }
        }

        res
    }
}