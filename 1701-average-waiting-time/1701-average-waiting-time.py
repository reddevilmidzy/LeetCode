class Solution:
    def averageWaitingTime(self, customers: List[List[int]]) -> float:
        res = 0
        cur = customers[0][0]
        for st, diff in customers:
            res += max(0, cur - st) + diff
            cur += diff
        return res / len(customers)
