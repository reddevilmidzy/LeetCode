class Solution:
    def minOperations(self, boxes: str) -> List[int]:
        n = len(boxes)
        go_pre = [0] # go_pre[i] = i-1 까지의 1 갯수
        back_pre = [0]
        for i in range(n):
            go_pre.append(go_pre[-1] + (boxes[i] == '1'))
            back_pre.append(back_pre[-1] + (boxes[n - i - 1] == '1'))

        go_dp = [0] * n
        back_dp = [0] * n
        for i in range(1, n):
            go_dp[i] = go_dp[i - 1] + go_pre[i]
            back_dp[i] = back_dp[i - 1] + back_pre[i]

        return [go_dp[i] + back_dp[n-i-1] for i in range(n)]