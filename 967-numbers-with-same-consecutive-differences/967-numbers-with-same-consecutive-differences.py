class Solution:
    def numsSameConsecDiff(self, n: int, k: int) -> List[int]:
        s=[]
        res = []
        nums = [1,2,3,4,5,6,7,8,9,0]
        def backtraking():
            if len(s)==n:
                c = int(''.join(list(map(str,s))))
                if len(str(c))==n:
                #print(c)
                    res.append(c)
                return res
            for i in nums:
                if len(s)==0:
                    s.append(i)
                    backtraking()
                    s.pop()
                else:
                    if s[-1] - i == k or i - s[-1] == k:
                        s.append(i)
                        backtraking()
                        s.pop()
                    else:
                        continue
        backtraking()
        res.sort()
        return res
                
        #print(res)