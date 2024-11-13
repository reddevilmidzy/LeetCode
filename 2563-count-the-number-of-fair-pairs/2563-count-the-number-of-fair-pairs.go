func upperBound(nums []int, start int, end int, target int) int {
	left := start
	right := end
	for left <= right {
		mid := left + (right-left)/2
		if nums[mid] > target {
			right = mid - 1
		} else {
			left = mid + 1
		}
	}
	return right
}

func lowerBound(nums []int, start int, end int, target int) int {
	left := start
	right := end
	for left <= right {
		mid := left + (right-left)/2
		if nums[mid] < target {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}
	return left
}

func countFairPairs(nums []int, lower int, upper int) int64 {
	var res int64 = 0
	sort.Ints(nums)
	n := len(nums)
	for i := 0; i < n; i++ {
		left := lowerBound(nums, i+1, n-1, lower-nums[i])
		right := upperBound(nums, i+1, n-1, upper-nums[i])
		if left <= right {
			res += int64(right - left + 1)
		}
	}
	return res
}
