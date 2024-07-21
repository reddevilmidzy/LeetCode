class Solution:
    def numberOfAlternatingGroups(self, colors: List[int], k: int) -> int:
        n = len(colors)
        i = 0
        j = 0
        cnt = 0
        ans = 0

        colors += colors

        while j < (n + k) - 1:
            if colors[j] == colors[j + 1]:
                i = j
            j += 1
            if j - i >= k:
                ans += 1
        
        return ans
