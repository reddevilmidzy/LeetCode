impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut r: usize = n - 1;
        while 0 < r && arr[r - 1] <= arr[r] {
            r -= 1;
        }
        let mut res = r;

        let mut l: usize = 0;
        while l + 1 < n && arr[l] <= arr[l + 1] {
            l += 1;
        }
        res = res.min(n - l - 1);

        l = 0;
        r = n - 1;

        while l < r {
            while r < n && l + 1 < r && arr[r - 1] <= arr[r] && arr[l] <= arr[r] {
                r -= 1;
            }
            while r < n && arr[l] > arr[r] {
                r += 1;
            }
            res = res.min(r - l - 1);
            if arr[l] > arr[l + 1] {
                break;
            }
            l += 1;
        }

        res as i32
    }
}
