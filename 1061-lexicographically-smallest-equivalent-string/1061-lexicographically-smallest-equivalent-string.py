from collections import deque

class Solution(object):
    def smallestEquivalentString(self, s1, s2, baseStr):
        
        def bfs(word,node):
            visited = [False]*26
            visited[ord(node)-97] = True
            queue = deque()
            queue.append(node)
            while queue:
                x = queue.popleft()
                for i in word[x]:
                    if not visited[ord(i)-97]:
                        queue.append(i)
                        visited[ord(i)-97] = True

            for i in range(26):
                if visited[i]:
                    return chr(i+97)

        def solve():
            n = len(s1)
            m = len(baseStr)
            res = ""
            word = defaultdict(list)
            for i in range(n):
                word[s1[i]].append(s2[i])
                word[s2[i]].append(s1[i])
            
            for i in range(m):
                res += bfs(word,baseStr[i])
            return res

        return solve()    