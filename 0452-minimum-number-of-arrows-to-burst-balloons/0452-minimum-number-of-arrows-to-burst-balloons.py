class Solution:
    def findMinArrowShots(self, points: List[List[int]]) -> int:
        points.sort(key=lambda x: (x[1], x[0]))
        res = 1
        last = points[0][1]
        for st,ed in points:
            if st <= last <= ed:
                pass
            else:
                last = ed
                res += 1
        return res