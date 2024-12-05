class Solution:
    def canChange(self, start: str, target: str) -> bool:
        n = len(start)
        i = 0
        j = 0
        if start.count("L") != target.count("L") or start.count("R") != target.count("R"):
            return False
        
        l_cnt = 0
        r_cnt = 0
        while i < n and j < n:
            if target[j] == "L": l_cnt += 1
            
            if start[i] == "R": r_cnt += 1
            
            if start[i] == "L":
                l_cnt -= 1
                if r_cnt: return False
                if l_cnt < 0: return False

            if target[j] == "R":
                r_cnt -= 1
                if l_cnt: return False
                if r_cnt < 0: return False
            
            i += 1
            j += 1

        return r_cnt==0 and l_cnt==0