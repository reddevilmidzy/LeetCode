# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def amountOfTime(self, root: Optional[TreeNode], start: int) -> int:
        
        queue = deque()
        queue.append(root)
        graph = defaultdict(list)        
        visited = {root.val}
        
        while queue:
            sub = queue.popleft()
            
            if not sub: continue
            elif type(sub) == int:
                if sub not in visited:
                    queue.append(sub)
                    visited.add(sub)
                continue
            
            if sub.left and sub.left.val not in visited:
                queue.append(sub.left)
                graph[sub.val].append(sub.left.val)
                graph[sub.left.val].append(sub.val)
                visited.add(sub.left.val)
            if sub.right and sub.right.val not in visited:
                queue.append(sub.right)
                graph[sub.val].append(sub.right.val)
                graph[sub.right.val].append(sub.val)
                visited.add(sub.right.val)

        queue.append((start,0))
        visited.clear()
        visited.add(start)
        while queue:
            cur,cnt = queue.popleft()
            
            for nxt in graph[cur]:
                if nxt not in visited:
                    queue.append((nxt, cnt+1))
                    visited.add(nxt)
        
        
        return cnt