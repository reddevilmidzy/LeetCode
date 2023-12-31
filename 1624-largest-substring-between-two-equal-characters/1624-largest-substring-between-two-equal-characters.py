class Solution:
    def maxLengthBetweenEqualCharacters(self, s: str) -> int:
        dist = [-1]*26
        alp = lambda x: ord(x) - ord('a')
        n = len(s)
        res = -1
        for i in range(n):
            if dist[alp(s[i])] == -1:
                dist[alp(s[i])] = i
            else:
                res = max(res, i - dist[alp(s[i])] - 1)
        return res