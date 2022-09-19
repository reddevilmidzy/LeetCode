class Solution:
    def findDuplicate(self, paths: List[str]) -> List[List[str]]:
        dic = dict()
        for i in paths:
            tmp = i.split(" ")
            for j in tmp[1:]:
                fp, cont = j.split("(")
                cont = cont[:-1] # 뒷 괄호 제거
                fullpath = "/".join([tmp[0],fp])
                dic[cont] = dic.get(cont,[])+[fullpath]
        return [v for v in dic.values() if len(v) >1]