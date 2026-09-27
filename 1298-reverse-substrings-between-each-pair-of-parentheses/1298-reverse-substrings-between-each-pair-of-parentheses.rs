impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stk = Vec::new();
        for c in s.chars() {
            match c {
                '(' => {
                    stk.push("(".to_string());
                },
                ')' => {
                    let mut tmp: Vec<String> = Vec::new();
                    while let Some(v) = stk.pop() {
                        if v == "(" {
                            break;
                        }
                        tmp.push(v.chars().rev().collect());
                    }
                    // tmp.reverse();
                    let st = tmp.into_iter().collect::<String>();
                    stk.push(st);
                },
                'a'..='z' => {
                    stk.push(c.to_string());
                },
                _ => unreachable!()
            }
            // println!("stk = {stk:?}");

        }
        // println!("final = {stk:?}");
        
        stk.into_iter().collect::<String>()
    }
}
