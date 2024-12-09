func isArraySpecial(nums []int, queries [][]int) []bool {
    n := len(nums)
    arr := make([]int, n)

    for i := 0; i <n-1; i++ {
        if (nums[i]%2 == nums[i+1]%2) {
            arr[i+1] = 1
        }
        arr[i+1] += arr[i]
    }

    res := []bool{}
    for _, query := range queries {
        res = append(res, arr[query[1]] - arr[query[0]] == 0)
    }
    return res
}