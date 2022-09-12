class Solution:
    def bagOfTokensScore(self, tokens: List[int], power: int) -> int:
        res,cur = 0,0
        queue = deque(sorted(tokens))
        # 정렬한 다음에 가장 싼것을 구매
        while queue and (queue[0] <= power or cur):
            if queue[0] <= power:
                power -= queue.popleft()
                cur += 1
            else:
                power += queue.pop()
                cur -= 1
            res = max(res, cur)
        
        return res