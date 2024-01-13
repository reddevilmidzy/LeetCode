class Solution:
    def minSteps(self, s: str, t: str) -> int:
        cnt = Counter(t)
        res = 0
        for st in s:
            if st in cnt and cnt[st] > 0:
                cnt[st] -= 1
                res += 1
        
        return len(s) - res