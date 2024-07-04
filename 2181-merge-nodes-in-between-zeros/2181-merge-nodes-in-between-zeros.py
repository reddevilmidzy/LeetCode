# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        tmp = self.searchNodes(head)
        if len(tmp) == 0:
            return None

        res = ListNode(tmp[0])
        nxt = None

        for i in range(1, len(tmp)):
            nxt = ListNode(tmp[i])
            res.nxt = nxt


        return res



    
    def searchNodes(self, head: Optional[ListNode]) -> List[int]:
        res = []
        cur = head
        tmp = 0
        while cur != None:
            if cur.val != 0:
                tmp += cur.val
            elif tmp != 0:
                res.append(tmp)
                tmp = 0
                
            cur = cur.next

        return res
