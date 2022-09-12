class Solution:
    def bagOfTokensScore(self, tokens: List[int], power: int) -> int:
        res,cur = 0,0
        d = collections.deque(sorted(tokens))
        # 정렬한다음에 가장 싼것을 구매
        # 결과값 리턴
        while d and (d[0] <= power or cur):
            if d[0] <= power:
                power -= d.popleft()
                cur += 1
            else:
                power += d.pop()
                cur -= 1
            res = max(res, cur)
        
        return res