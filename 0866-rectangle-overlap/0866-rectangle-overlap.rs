impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        fn between(a: i32, b: i32, c: i32, d: i32) -> bool {
            (a < b && b < c) || (a < d && d < c) 
                || (b < a && a < d) || (b < c && c < d)
        }
        let [ax1, ay1, ax2, ay2] = rec1[..] else { todo!() };
        let [bx1, by1, bx2, by2] = rec2[..] else { todo!() };
        if rec1 == rec2 {
            return true;
        }
        (between(ax1, bx1, ax2, bx2)) && (between(ay1, by1, ay2, by2))
    }
}
