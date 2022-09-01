# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        if not root: return 0 # 들어오는게 없다면 리턴 0
        self.res = 0 # 정답 변수

        def sol_rec(cur, val):
            if cur.val >= val: self.res += 1 # 굿노드 조건에 만족한다면
            if cur.left: sol_rec(cur.left, max(val, cur.val)) # 좌측 자식 노드 탐색
            if cur.right: sol_rec(cur.right, max(val, cur.val)) # 우측 자식 노드 탐색

        sol_rec(root, root.val)
        return self.res
#         if not root: return 0
#         self.ans = 0
        
#         def cnt_good(cur, val):
#             if cur.val >= val: # 굿 노드라면 ans 추가
#                 self.ans += 1
#             if cur.left: cnt_good(val, max(val, cur.val))
#             if cur.right: cnt_good(val, max(val, cur.val))
        
#         cnt_good(root, root.val)
#         return ans    