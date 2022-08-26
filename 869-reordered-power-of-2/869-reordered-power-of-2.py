class Solution:
    def reorderedPowerOf2(self, n: int) -> bool:
        two = 1
        n = str(n)
        dict_two = defaultdict(list)
        while len(str(two)) <= len(n):
            dict_two[len(str(two))].append(two)
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
            