class Solution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        for i in ransomNote:
            if i in magazine:
                magazine = magazine.replace(i,'',1)
                # print(magazine,i)
            else:
                #print('i',i)
                return False
        return True
        