impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        use std::collections::HashSet;
        let mut res = HashSet::new();
        let n = n as usize;
        
        fn check(cur: &Vec<Vec<bool>>, y: usize, x: usize) -> bool {
            let n = cur.len();
            let nn = n as i32;
            let yy = y as i32;
            let xx = x as i32;
            for (dy, dx) in [(-1,-1), (1,-1), (-1,1), (1,1)] {
                for i in 1..nn {
                    let ny = yy + dy * i;
                    let nx = xx + dx * i;
                    if ny < 0 || nx < 0 || ny >= nn || nx >= nn {
                        break;
                    }

                    let ny = ny as usize;
                    let nx = nx as usize;
                    if cur[ny][nx] {
                        return false;
                    }
                }
            }
            (0..n).all(|i| !cur[i][x])
        }

        fn solve(m: usize, n: usize, nxt_y: usize, cur: &mut Vec<Vec<bool>>, res: &mut HashSet<Vec<String>>) {
            if n == m {
                let mut tmp = Vec::with_capacity(n);
                for i in 0..n {
                    let mut t = String::with_capacity(n);
                    for j in 0..n {
                        if cur[i][j] {
                            t.push('Q');
                        } else {
                            t.push('.');
                        }
                    }
                    tmp.push(t);
                }
                res.insert(tmp);
                return;
            }
            for y in nxt_y..n {
                for x in 0..n {
                    if check(&cur, y, x) {
                        cur[y][x] = true;
                        solve(m + 1, n, y + 1, cur, res);
                        cur[y][x] = false;
                    }
                }
            }
        }

        solve(0, n, 0, &mut vec![vec![false; n]; n], &mut res);
        res.into_iter().collect::<Vec<_>>()
    }
}
