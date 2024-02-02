class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        dag = [0]*numCourses
        graph = [[] for _ in range(numCourses)]
        for a, b in prerequisites:
            dag[a] += 1
            graph[b].append(a)

        queue = deque()
        for v in range(numCourses):
            if dag[v] == 0:
                queue.append(v)
        if len(queue) == 0: return False

        while queue:
            cur = queue.popleft()            
            for nxt in graph[cur]:
                dag[nxt] -= 1
                if dag[nxt] == 0:
                    queue.append(nxt)

        return sum(dag) == 0