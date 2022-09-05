"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""

class Solution:
    def levelOrder(self, root: 'Node') -> List[List[int]]:
        q, ret = [root], [] # 기본 원소 담겨있는배열, 정답배열
        while any(q): # q가 비어있다면 멈춤
            ret.append([node.val for node in q])
            q = [child for node in q for child in node.children if child] # q에 있는 node 의 children 이 비어있지 않다면
        return ret