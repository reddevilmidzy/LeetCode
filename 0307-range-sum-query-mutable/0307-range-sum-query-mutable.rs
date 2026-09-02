struct NumArray {
    n: usize,
    arr: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn build(&mut self, nums: &[i32], node: usize, lo: usize, hi: usize) {
        if hi - lo == 1 {
            self.arr[node] = nums[lo];
            return;
        }
        let mid = (lo + hi) >> 1;
        self.build(nums, node << 1, lo, mid);
        self.build(nums, node << 1 | 1, mid, hi);

        self.arr[node] = self.arr[node << 1] + self.arr[node << 1 | 1];
    }

    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut sg =  Self {n, arr: vec![0; 4 * n], };
        sg.build(&nums, 1, 0, n);
        sg
    }

    fn update(&mut self, index: i32, val: i32) {
        self.update_query(1, 0, self.n, index as usize, val);
    }

    fn update_query(&mut self, node: usize, st: usize, ed: usize, idx: usize, val: i32) {
        if ed - st == 1 {
            self.arr[node] = val;
            return;
        }

        let mid = (st + ed) >> 1;
        if idx < mid {
            self.update_query(node << 1, st, mid, idx, val);
        } else {
            self.update_query(node << 1 | 1, mid, ed, idx, val);
        }
        self.arr[node] = self.arr[node << 1] + self.arr[node << 1 | 1];
    }
    
    // [left, right]
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        assert!(left < right);
        assert!(right <= self.n as i32);

        self.sum_query(1, 0, self.n, left as usize, right as usize + 1)
    }

    fn sum_query(&self, node: usize, st: usize, ed: usize, l: usize, r: usize) -> i32 {
        if r <= st || ed <= l {
            return 0;
        }
        if l <= st && ed <= r {
            return self.arr[node];
        }

        let mid = (st + ed) >> 1;
        self.sum_query(node << 1, st, mid, l, r) + self.sum_query(node << 1 | 1, mid, ed, l, r)
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */