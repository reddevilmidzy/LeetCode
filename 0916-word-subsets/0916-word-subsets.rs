impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();

        let mut cnt2: Vec<u32> = vec![0; 26];

        for word in words2 {
            let tmp = Self::counter(word);
            for i in 0..26 {
                cnt2[i] = cnt2[i].max(tmp[i]);
            }
        }

        for word in words1 {
            let cnt1: Vec<u32> = Self::counter(word.clone());

            if Self::is_subset(&cnt1, &cnt2) {
                res.push(word);
            }
        }

        res
    }

    pub fn is_subset(cnt1: &Vec<u32>, cnt2: &Vec<u32>) -> bool {
        for i in 0..26 {
            if cnt1[i] < cnt2[i] {
                return false;
            }
        }
        true
    }

    pub fn counter(word: String) -> Vec<u32> {
        let mut cnt: Vec<u32> = vec![0; 26];

        for c in word.chars() {
            cnt[(c as u8 - 'a' as u8) as usize] += 1;
        }
        cnt
    }
}