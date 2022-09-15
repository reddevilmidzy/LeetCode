class Solution:
    def findOriginalArray(self, changed: List[int]) -> List[int]:
        n = len(changed)
        
        if n%2 != 0:
            return []
        
        changed.sort()
        c = Counter(changed)
        answer = []
        
        for num in changed:
            if c[num] >= 1:
                c[num] -= 1
                if c[num * 2] >= 1:
                    answer.append(num)
                    c[num * 2] -= 1
                
        if len(answer) == n// 2:
            return answer
        
        return []