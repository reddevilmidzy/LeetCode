impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes: Vec<i32> = boxes.chars().map(|x| x as i32 - '0' as i32).collect();
        let n: usize = boxes.len();
        let mut res: Vec<i32> = vec![0; n];

        let mut move_left = 0; // 현재 값
        let mut ball_left = 0; // 하나 이전 보관용
        let mut move_right = 0;
        let mut ball_right = 0;

        for i in 0..n {
            res[i] += move_left;
            ball_left += boxes[i];
            move_left += ball_left;

            res[n-i-1] += move_right;
            ball_right += boxes[n-i-1];
            move_right += ball_right;
        }

        res
    }
}