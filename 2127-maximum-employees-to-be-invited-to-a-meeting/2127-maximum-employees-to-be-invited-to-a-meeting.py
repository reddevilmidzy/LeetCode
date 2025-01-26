class Solution:
    def maximumInvitations(self, favorite: List[int]) -> int:
        n = len(favorite)
        visited = [False] * n
        length_2_cycles = []
        longest_cycle = 0
        
        for i in range(n):
            if visited[i]: continue
        
            st,cur = i, i
            cur_set = set()
            while not visited[cur]:
                visited[cur] = True
                cur_set.add(cur)
                cur = favorite[cur]
            if cur in cur_set:
                length = len(cur_set)
                while st != cur:
                    length -= 1
                    st = favorite[st]
                longest_cycle = max(longest_cycle, length)
                if length == 2:
                    length_2_cycles.append([cur, favorite[cur]])
        
        inverted = [[] for _ in range(n)]

        for dst, src in enumerate(favorite):
            inverted[src].append(dst)
        
        def bfs(src, parent):
            q = deque([(src, 0)])
            max_length = 0

            while q:
                node, length = q.popleft()
                if node == parent: continue

                max_length = max(max_length, length)
                for nxt in inverted[node]:
                    q.append((nxt, length + 1))
                
            return max_length

        chain_sum = 0

        for a,b in length_2_cycles:
            chain_sum += bfs(a, b) + bfs(b, a) + 2
        
        return max(chain_sum, longest_cycle)
