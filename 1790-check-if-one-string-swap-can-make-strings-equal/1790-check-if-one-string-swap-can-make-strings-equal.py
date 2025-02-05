class Solution:
    def areAlmostEqual(self, s1: str, s2: str) -> bool:
        if s1 == s2:
            return True
        
        n = len(s1)
        first = -1
        second = -1
        for i in range(n):
            if s1[i] != s2[i]:
                if first == -1:
                    first = i
                elif second == -1:
                    second = i
                else:
                    return False
        return s1[first] == s2[second] and s1[second] == s2[first]
