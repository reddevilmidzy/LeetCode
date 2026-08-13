impl Solution {
    pub fn max_area(mat: Vec<Vec<i32>>) -> i32 {

        fn can(n: usize, m: usize, k: usize, pre: &Vec<Vec<i32>>) -> bool {
            let mut candy = Vec::new();
            let kk = ((k + 1) * (k + 1)) as i32;
            
            for y in 0..n {
                for x in 0..m {
                    // y,x ny,nx
                    let ny = y + k;
                    let nx = x + k;
                    if ny >= n || nx >= m {
                        continue;
                    }
                    let cur = pre[ny + 1][nx + 1] - pre[ny + 1][x] - pre[y][nx + 1] + pre[y][x];
                    let is_squre = kk == cur;
                    if is_squre {
                        candy.push((y as i32, x as i32, ny as i32, nx as i32));
                    }
                }
            }
            if candy.len() < 2 {
                return false;
            }

            for i in 0..candy.len() {
                for j in (i+1..candy.len()).rev() {
                    let (ay, ax, any, anx) = candy[i];
                    let (by, bx, bny, bnx) = candy[j];
                    if !(by <= any && bx <= anx) || !(bny >= ay && bnx >= ax) {
                        return true;
                    }
                }
            }
            false
        }

        let mut res = 0;
        let n = mat.len();
        let m = mat[0].len();
        let mut pre = vec![vec![0; m + 1]; n + 1];
        let mut tmp = 0;

        for i in 0..n {
            for j in 0..m {
                pre[i + 1][j + 1] = pre[i][j + 1] + pre[i + 1][j] - pre[i][j] + mat[i][j];
                tmp += mat[i][j];
            }
        }

        // println!("{n}, m {m}");

        if tmp > 1 {
            res = 1;
        } else {
            return 0;
        }
        if n == 1 && m == 1 {
            return 0;
        } else if n == 1 || m == 1 {
            return if tmp >= 2 { 1 } else { 0 };
        }
        if tmp as usize == n * m {
            let val = (n.max(m) / 2) as i32;
            return val * val;
        }

        let mut hi = n.max(m) / 2 + 1;
        let mut lo = 0;

        while lo + 1 < hi {
            let mid = (lo + hi) / 2;
            // println!("mid = {mid} lo = {lo} hi = {hi}");
            if can(n, m, mid, &pre) {
                // println!("can mid = {mid}");
                lo = mid;
            } else {
                hi = mid;
            }
        }
        // println!("ed lo = {lo} hi = {hi}");
        (hi * hi) as i32
    }
}
