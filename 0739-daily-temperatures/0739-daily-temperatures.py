class Solution:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        n = len(temperatures)
        stk = []
        res = [0]*n
        for i in range(n):
            while stk and temperatures[stk[-1]] < temperatures[i]:
                val = stk.pop()
                res[val] = i - val
            stk.append(i)

        return res