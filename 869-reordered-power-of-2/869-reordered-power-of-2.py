class Solution:
    def reorderedPowerOf2(self, n: int) -> bool:
        two = 1
        n = str(n)
        dict_two = dict()
        while len(str(two)) <= len(n):
            if len(str(two)) in dict_two:
                dict_two[len(str(two))].append(two)
            else:
                dict_two[len(str(two))] = [two]
                
            two *= 2
        
        for num in dict_two[len(n)]:
            ans = str(num)
            for i in range(len(n)):
                print(n[i], num)
                if n[i] in ans:
                    ans = ans.replace(n[i], '', 1)
            if ans == '':
                return True
            
        return False
            