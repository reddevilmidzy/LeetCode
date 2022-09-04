# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def verticalTraversal(self, root: Optional[TreeNode]) -> List[List[int]]:
        # 아마 매핑을 위한 dict 자료형
        mapping = collections.defaultdict(list) 
        # 큐
        queue = collections.deque([(root, 0)])
        x_min, x_max = 0, 0
        while queue: # 큐가 빌때까지
            tmp = collections.defaultdict(list) # 이것도 역시 초기값은 list 로 된 dict
            for _ in range(len(queue)):
                node, x = queue.popleft()
                tmp[x].append(node.val) # 이 node.val 이 뭔지 모르겠음
                if node.left:
                    queue.append((node.left, x-1))
                    x_min = min(x_min, x-1)
                if node.right: 
                    queue.append((node.right, x+1)) 
                    x_max = max(x_max, x+1)
            for i in tmp: 
                mapping[i] += sorted(tmp[i])
        return [mapping[i] for i in range(x_min, x_max+1)]