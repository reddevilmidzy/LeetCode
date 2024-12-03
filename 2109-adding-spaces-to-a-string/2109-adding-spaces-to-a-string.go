func addSpaces(s string, spaces []int) string {
    res := make([]uint8, 0, len(s) + len(spaces))

    for s_idx, sp_idx := 0, 0; s_idx < len(s); s_idx++ {
        if sp_idx < len(spaces) && s_idx == spaces[sp_idx] {
            res = append(res, ' ')
            sp_idx++
        }
        res = append(res, s[s_idx])
    }

    return string(res)
}