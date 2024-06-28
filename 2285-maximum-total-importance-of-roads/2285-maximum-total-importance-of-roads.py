class Solution:
    def maximumImportance(self, n: int, roads: List[List[int]]) -> int:
        res = 0
        cnt = {i:0 for i in range(n)}

        for u,v in roads:
            cnt[u] += 1
            cnt[v] += 1

        num = n
        for val in sorted(cnt.values(), key=lambda x: -x):
            res += num*val
            num -= 1
        return res
