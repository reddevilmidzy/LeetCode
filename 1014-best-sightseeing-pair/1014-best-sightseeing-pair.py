class Solution:
    def maxScoreSightseeingPair(self, values: List[int]) -> int:
        cur,res = 0,0
        for v in values:
            res = max(res, cur+v) # 결과는 현재냐 아니면 전에거 추가냐
            cur = max(cur, v) -1
            
        return res