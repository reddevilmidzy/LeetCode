impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut res = 0f64;
        let n = points.len();

        for i in 0..n {
            for j in i+1..n {
                for k in j+1..n {
                    res = res.max(Self::cal_area(&points[i], &points[j], &points[k]));
                }
            }
        }

        res
    }

    fn cal_area(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>) -> f64 {
        (a[0]*b[1] + b[0]*c[1] + c[0]*a[1] - a[1]*b[0] - b[1]*c[0] - c[1]*a[0]).abs() as f64 * 0.5
    }
}
