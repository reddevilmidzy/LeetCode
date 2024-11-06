class Solution {
    fun canSortArray(nums: IntArray): Boolean {
        var curBits = 0
        var preMax = 0
        var max = 0

        for (num in nums) {
            var bits = num.countOneBits()
            if (bits != curBits) {
                curBits = bits
                preMax = max
            }
            if (num < preMax) return false
            max = max(max, num)
        }
        return true
    }
}
