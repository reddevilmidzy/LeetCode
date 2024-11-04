impl Solution {
    pub fn compressed_string(word: String) -> String {
        word.into_bytes()
            .chunk_by(|a, b| a==b)
            .flat_map(|chunk| {
                chunk
                    .chunks(9)
                    .flat_map(|c| [Into::<char>::into(c.len() as u8 + b'0'), chunk[0].into()])
            })
            .collect()
    }
}
