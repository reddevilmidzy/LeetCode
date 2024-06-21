class Solution:
    def maxSatisfied(self, customers: List[int], grumpy: List[int], minutes: int) -> int:
        n = len(customers)
        res = 0
        pre = [0]

        for i in range(n):
            if not grumpy[i]:
                res += customers[i]
                pre.append(pre[-1])
            else:
                pre.append(pre[-1] + customers[i])

        mv = 0
        for i in range(minutes, n + 1):
            mv = max(mv, pre[i] - pre[i-minutes])

        return res + mv
