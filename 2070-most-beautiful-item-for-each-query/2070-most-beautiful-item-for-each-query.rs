impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort();
        let n = items.len();
        let mut mv = 0;
        for item in items.iter_mut().take(n) {
            mv = mv.max(item[1]);
            item[1] = mv;
        }

        let mut res = Vec::with_capacity(queries.len());

        for query in queries {
            let mut st = 0;
            let mut ed = n as i32 - 1;

            let mut max_beauty = 0;
            while st <= ed {
                let mid = (st + ed) / 2;
                let found = items[mid as usize][0];

                if found > query {
                    ed = mid - 1;
                } else {
                    max_beauty = max_beauty.max(items[mid as usize][1]);
                    st = mid + 1;
                }
            }
            res.push(max_beauty);
        }
        res
    }
}
