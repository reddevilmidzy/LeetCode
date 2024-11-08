impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_by_key(|x| (-x[0], x[1]));
        let mut res = Vec::with_capacity(people.len());
        for person in people.iter() {
            res.insert(person[1] as usize, person.to_vec());
        }
        res
    }
}
