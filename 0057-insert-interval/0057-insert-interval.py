class Solution:
    def insert(self, intervals: list[list[int]], newInterval: list[int]) -> list[list[int]]:
        res = intervals + [newInterval]
        res.sort(key=lambda x: x[0])
        ans = [res[0]]
        for st, ed in res[1:]:
            if ans[-1][1] >= st:
                ans[-1][1] = max(ans[-1][1], ed)
            else:
                ans.append([st, ed])
        return ans