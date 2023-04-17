class Solution:
    def kidsWithCandies(self, candies: List[int], extraCandies: int) -> List[bool]:
        res = []
        max_candy = max(candies)
        for candy in candies:
            if candy + extraCandies >= max_candy:
                res.append(True)
            else:
                res.append(False)
        return res