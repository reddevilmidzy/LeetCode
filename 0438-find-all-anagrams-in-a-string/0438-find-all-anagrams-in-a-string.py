class Solution:
    def findAnagrams(self, s: str, p: str) -> List[int]:
        chars = [0]*26
        charp = [0]*26
        
        if len(p) >len(s):
            return []
        
        for x in p:
            charp[ord(x) - ord('a')] += 1
        
        for i in range (len(p)):
            chars[ord(s[i]) - ord('a')] += 1
        
        l, r = 0, len(p) - 1
        res = []
        
        while r < len(s):
            if chars == charp:
                res.append(l)
            chars[ord(s[l]) - ord('a')] -=1
            l, r = l+1, r+1
            if r >= len(s):
                break
            chars[ord(s[r]) - ord('a')] += 1
        return res