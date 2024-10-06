class Solution:
    def areSentencesSimilar(self, sentence1: str, sentence2: str) -> bool:
        s1=sentence1.split()
        s2=sentence2.split()
        n = len(s1)
        m = len(s2)

        if n==m:
            return s1==s2
        
        def solve(s1: List[str], s2: List[str]) -> bool:
            i = 0
            j = 0
            cnt = 0
            same = True
            while i < n and j < m:
                if s1[i] == s2[j]:
                    same = True
                    i+=1
                    j+=1
                else:
                    if same:
                        cnt += 1
                    same = False
                    if n > m:
                        i += 1
                    else:
                        j += 1

            print('i',i,'j',j,'cnt',cnt)
            if cnt == 0:
                return i == n or j == m
            return i==n and j==m and cnt == 1

        return solve(s1, s2) or solve(s1[::-1], s2[::-1])