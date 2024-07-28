class Solution:
    def canVisitAllRooms(self, rooms: List[List[int]]) -> bool:
        n = len(rooms)
        visited = [False] * n
        visited[0] = True
        queue = deque([0])

        while queue:
            cur = queue.popleft()

            for nxt in rooms[cur]:
                if not visited[nxt]:
                    queue.append(nxt)
                    visited[nxt] = True
        
        return all(num for num in visited)
    