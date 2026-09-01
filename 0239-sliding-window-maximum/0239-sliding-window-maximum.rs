#[derive(Debug)]
struct SparseTable {
    table: Vec<Vec<i32>>,
    log2: Vec<usize>,
}

impl SparseTable {
    fn new(nums: &[i32]) -> Self {
        let n = nums.len();
        // log2[i] = floor(log2(i))
        let mut log2 = vec![0; n + 1];
        for i in 2..=n {
            log2[i] = log2[i / 2] + 1;
        }
        let lev = log2[n] + 1;
        let mut table = vec![vec![0; n]; lev];

        table[0].copy_from_slice(nums);

        for l in 1..lev {
            let len = 1 << l;
            let half = len >> 1;

            for st in 0..=n-len {
                table[l][st] = table[l - 1][st].max(table[l - 1][st + half]);
            }
        }

        Self { table, log2 }
    }

    // [left, right) êµ¬ê°
    fn query(&self, left: usize, right: usize) -> i32 {
        let len = right - left;
        let lev = self.log2[len];
        let block = 1 << lev;

        self.table[lev][left].max(self.table[lev][right - block])
    }
}
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;

        let st = SparseTable::new(&nums);
        // println!("{st:?}");
        let mut res = Vec::with_capacity(n - k + 1);

        for left in 0..=n - k {
            res.push(st.query(left, left + k));
        }
        
        res
    }
}
