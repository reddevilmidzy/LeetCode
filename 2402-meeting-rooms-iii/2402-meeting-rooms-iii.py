class Solution:
    def mostBooked(self, n: int, meetings: List[List[int]]) -> int:
        rooms, count = [0] * n, [0] * n
        for a, b in sorted(meetings):
            best_i, best_end = None, inf
            for i, end in enumerate(rooms):
                if a >= end:
                    rooms[i] = b
                    count[i] += 1
                    break
                if end < best_end:
                    best_i, best_end = i, end
            else:
                rooms[best_i] = b - a + rooms[best_i]
                count[best_i] += 1
        return count.index(max(count))