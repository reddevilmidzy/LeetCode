class Solution:
    def canChange(self, start: str, target: str) -> bool:
        l_cnt = 0
        r_cnt = 0
        for i in range(len(start)):
            if target[i] == "L": l_cnt += 1
            
            if start[i] == "R": r_cnt += 1
            
            if start[i] == "L":
                l_cnt -= 1
                if r_cnt: return False
                if l_cnt < 0: return False

            if target[i] == "R":
                r_cnt -= 1
                if l_cnt: return False
                if r_cnt < 0: return False
            
        return r_cnt==0 and l_cnt==0