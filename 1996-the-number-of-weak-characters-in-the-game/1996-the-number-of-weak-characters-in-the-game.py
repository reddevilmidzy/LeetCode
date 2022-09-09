class Solution:
    def numberOfWeakCharacters(self, properties: List[List[int]]) -> int:
        properties = sorted(properties, key=lambda x: (-x[0],x[1]))
        
        weak = 0
        cur_max = 0
        
        for a,d in properties:
            if d < cur_max:
                weak +=1
            else:
                cur_max = d
        return weak