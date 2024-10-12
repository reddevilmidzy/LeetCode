class Solution:
    def minGroups(self, intervals: List[List[int]]) -> int:
        # intervals.sort(key=lambda x:x[-1])
        intervals.sort()
        res = []
        for st,ed in intervals:
            if not res or res[0][0] >= st:
                heapq.heappush(res, (ed, st))
            else:
                heapq.heapreplace(res, (ed, st))

        return len(res)
