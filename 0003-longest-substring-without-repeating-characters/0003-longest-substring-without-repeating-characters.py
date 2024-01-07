class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        if not s: return 0
        n = len(s)
        res = 1
        cnt = defaultdict(int)
    
        l,r = 0, 1
        pre = 0
        cnt[s[l]] = 1
        while r < n:
            
            if l != pre:
                res = max(res, r - l + 1)
                pre = l

            if cnt[s[r]] < 1:
                cnt[s[r]] += 1
                r += 1
            else:
                cnt[s[l]] -= 1
                l += 1
        
        if max(cnt.values()) < 2:
            res = max(res, r - l)
        return res