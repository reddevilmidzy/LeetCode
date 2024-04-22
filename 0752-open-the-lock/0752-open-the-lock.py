class Solution:
    def openLock(self, deadends: List[str], target: str) -> int:
        deadends = set([int(num) for num in deadends])
        queue = deque()
        visited = set()
        visited.add(0)
        queue.append((0, 0))
        target = int(target)
        if 0 in deadends:
            return -1
        while queue:
            cur, cnt = queue.popleft()
            if cur == target:
                return cnt
            for i in range(4):
                if cur//(10**i) % 10 == 9:
                    nxt = cur - (10**i) * 9
                else:
                    nxt = cur + (10 ** i)
                if nxt not in deadends and nxt not in visited and 0 <= nxt <= 9999:
                    queue.append((nxt, cnt + 1))
                    visited.add(nxt)

                if cur//(10**i) % 10 == 0:
                    nxt = cur + (10 ** i) * 9
                else:
                    nxt = cur - (10 ** i)
                if nxt not in deadends and nxt not in visited and 0 <= nxt <= 9999:
                    queue.append((nxt, cnt + 1))
                    visited.add(nxt)

        return -1
