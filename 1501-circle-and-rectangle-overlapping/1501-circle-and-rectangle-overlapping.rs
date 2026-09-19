impl Solution {
    pub fn check_overlap(r: i32, xc: i32, yc: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        // ì°ë¦¬ê° êµ¬í´ì¼ íë ê²ì
        // ìê³¼ ì¬ê°íì´ ê²¹ì¹ëì§ íìíëê²
        // (x-xc)^2 + (y-yc)^2 = r^2

        // x1 <= x <= x2
        // y1 <= y <= y2
        let r = r.pow(2);

        for y in y1..=y2 {
            for x in x1..=x2 {
                let nr = (xc-x).pow(2) + (yc-y).pow(2);
                // println!("({y},{x}) nr = {nr}");
                if nr <= r {
                    return true;
                }
            }
        }
        false
    }
}
