class Solution:
    def countPalindromicSubsequence(self, s: str) -> int:
        res = 0
        n = len(s)
        first = [-1] * 26
        last = [-1] * 26
        for i in range(n):
            c = ord(s[i]) - ord('a')
            if first[c] == -1:
                first[c] = i
            last[c] = i
        
        for i in range(26):
            if first[i] == -1 or last[i] == -1 or first[i] == last[i]: continue
            tmp = set()
            for j in range(first[i] + 1, last[i]):
                tmp.add(s[j])

            res += len(tmp)

        return res
