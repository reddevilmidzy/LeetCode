class Solution:
    def maxUniqueSplit(self, s: str) -> int:
        res = 0
        n = len(s)

        def bt(idx, word):
            nonlocal res
            if idx == n-1:
                res = max(res, len(word))
                return 

            tmp = ''
            for i in range(idx + 1, n):
                tmp += s[i]
                if tmp not in word:
                    bt(i, word | {tmp})
   
        bt(-1, set())
        return res
    