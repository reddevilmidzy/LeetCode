#[derive(Debug)]
struct MergeSortTree {
    tree: Vec<Vec<i32>>,
}

impl MergeSortTree {
    fn new(nums: &Vec<i32>) -> Self {
        let n = nums.len();
        let mut tree = vec![Vec::new(); n * 4];
        
        let mut mst = Self { tree };
        mst.build(nums, 1, 0, n - 1);
        mst
    }

    fn build(&mut self, nums: &Vec<i32>, idx: usize, lo: usize, hi: usize) {
        if lo == hi {
            self.tree[idx] = vec![nums[lo]];
        } else {
            let mid = (lo + hi) >> 1;
            self.build(nums, idx << 1, lo, mid);
            self.build(nums, idx << 1 | 1, mid + 1, hi);
            
            let n = self.tree[idx << 1].len();
            let m = self.tree[idx << 1 | 1].len();

            let mut res = Vec::with_capacity(n + m);
            let mut i = 0;
            let mut j = 0;

            while i + j < n + m {
                if i == n {
                    res.push(self.tree[idx << 1 | 1][j]);
                    j += 1;
                } else if j == m {
                    res.push(self.tree[idx << 1][i]);
                    i += 1;
                } else {
                    if self.tree[idx << 1][i] <= self.tree[idx << 1 | 1][j] {
                        res.push(self.tree[idx << 1][i]);
                        i += 1;
                    } else {
                        res.push(self.tree[idx << 1 | 1][j]);
                        j += 1;
                    }
                }
            }
            self.tree[idx] = res;
        }
    }

    fn merge(&mut self, idx: usize, left: &Vec<i32>, right: &Vec<i32>) {
        let n = left.len();
        let m = right.len();

        let mut res = Vec::with_capacity(n + m);
        let mut i = 0;
        let mut j = 0;

        while i + j < n + m {
            if i == n {
                res.push(right[j]);
                j += 1;
            } else if j == m {
                res.push(left[i]);
                i += 1;
            } else {
                if left[i] <= right[j] {
                    res.push(left[i]);
                    i += 1;
                } else {
                    res.push(right[j]);
                    j += 1;
                }
            }
        }
        self.tree[idx] = res;
    }

    fn query(&self, idx: usize, val: i32, tl: usize, tr: usize, l: usize, r: usize) -> i32 {
        if l > r {
            return 0;
        }
        if l == tl && r == tr {
            return self.tree[idx].partition_point(|&x| x < val) as i32;
        }
        let tm = (tl + tr) >> 1;
        self.query(idx << 1, val, tl, tm, l, r.min(tm)) + self.query(idx << 1 | 1, val, tm + 1, tr, l.max(tm + 1), r)
    }
}

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mst = MergeSortTree::new(&nums);
        // println!("{mst:?}");
        (0..n).map(|i| mst.query(1, nums[i], 0, n - 1, i, n - 1)).collect::<Vec<_>>()
    }
}
