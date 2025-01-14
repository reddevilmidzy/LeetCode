use std::collections::HashSet;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();

        let mut a_set: HashSet<i32> = HashSet::with_capacity(n);
        let mut b_set: HashSet<i32> = HashSet::with_capacity(n);

        let mut res: Vec<i32> = vec![0; n];
        for i in 0..n {
            a_set.insert(a[i]);
            b_set.insert(b[i]);

            let mut tmp = a_set.intersection(&b_set);
            res[i] = tmp.count() as i32;
        }

        res
    }
}