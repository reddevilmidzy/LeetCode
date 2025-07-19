use std::collections::HashSet;

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        let mut res = Vec::new();
        let mut par = HashSet::new();
        folder.sort_by_key(|f| f.split("/").count());
        // println!("{folder:?}");
        par.insert(folder[0].clone());
        res.push(folder[0].clone());

        for i in 1..folder.len() {
            let mut cur = folder[i].split("/").collect::<Vec<_>>();
            cur.pop();
            let mut tmp = String::new();
            let mut flag = true;
            for j in 1..cur.len() {
                tmp.push_str("/");
                tmp.push_str(cur[j]);

                if par.contains(&tmp) {
                    flag = false;
                    break;
                }
            }

            if flag {
                res.push(folder[i].clone());
            }

            par.insert(folder[i].clone());
        }

        res
    }
}
