class Solution:
    def same(self, s: dict, t: dict) -> bool:
        for key in t.keys():
            if s[key] < t[key]:
                return False
        return True
    
    def minWindow(self, s: str, t: str) -> str:
        n = len(s)
        m = len(t)
        
        di = dict()
        t_di = dict()
        for v in t:
            if v not in t_di:
                t_di[v] = 0
            t_di[v] += 1
            di[v] = 0
        res = ''
        one, two = 0,0
    
        while one < n and s[one] not in t_di:
            one += 1
        
        two = one
        
        while one <= two <= n:
            # print("one", one, "two", two, di)
            if self.same(di , t_di):
                if res == '':
                    res = s[one:two]
                elif len(res) > len(s[one:two]):
                    res = s[one:two]
                di[s[one]] -= 1
                tmp = one + 1
                while tmp < two:
                    if s[tmp] in t_di:
                        break
                    tmp += 1

                one = tmp
                # print('same', 'one', one, 'tmp', tmp)
            else:
                if two >= n: break
                v = s[two]
                if v in t_di:
                    di[v] += 1
                two += 1
        
        if self.same(di, t_di):
            if res == '':
                res = s[one:two]
            elif len(res) > len(s[one:two]):
                res = s[one:two]
            # res.append(s[one:two])
        # print('one', one, 'two', two)
        # print(t_di)
        # print(res)
        # res.sort(key=lambda x:len(x))
        return res