func checkIfExist(arr []int) bool {
    visited := make(map[int]bool)
    for _, v := range arr {
        if visited[2*v] {
            return true
        }
        if v%2==0 && visited[v/2] {
            return true
        }
        visited[v] = true
    }
    return false
}
