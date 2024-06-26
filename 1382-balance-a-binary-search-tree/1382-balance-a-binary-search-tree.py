# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def __init__(self):
        self.nums = []

    def balanceBST(self, root: TreeNode) -> TreeNode:
        self.inorder(root)
        
        return self.build_bbst(self.nums)

    def inorder(self, root: TreeNode) -> None:
        if root == None:
            return
        self.inorder(root.left)
        self.nums.append(root.val)
        self.inorder(root.right)

    def build_bbst(self, nums: List[int]) -> TreeNode:
        if len(nums) == 0:
            return None
        mid = len(nums) // 2
        node = TreeNode(nums[mid])
        node.left = self.build_bbst(nums[:mid])
        node.right = self.build_bbst(nums[mid + 1:])
        return node
