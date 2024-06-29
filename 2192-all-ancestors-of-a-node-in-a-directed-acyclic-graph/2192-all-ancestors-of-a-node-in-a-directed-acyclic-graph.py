class Solution:
    def getAncestors(self, n: int, edges: List[List[int]]) -> List[List[int]]:
        graph = [[] for _ in range(n)]
        dag = [0]*n
        ancestors = [set() for _ in range(n)]
        for u,v in edges:
            graph[u].append(v)
            dag[v] += 1
        
        queue = deque()
        for i in range(n):
            if not dag[i]:
                queue.append(i)
        
        while queue:
            cur = queue.popleft()

            for nxt in graph[cur]:
                dag[nxt] -= 1
                ancestors[nxt].add(cur)
                ancestors[nxt] |= ancestors[cur]

                if not dag[nxt]:
                    queue.append(nxt)
        
        for i in range(n):
            ancestors[i] = sorted(ancestors[i])
        return ancestors
