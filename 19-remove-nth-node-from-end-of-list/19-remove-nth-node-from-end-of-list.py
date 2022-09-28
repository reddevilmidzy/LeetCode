# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        slow, fast = head, head
        # 슬로, 패슽
        while n > 0:
            fast = fast.next
            n -= 1
        
        while fast is not None and fast.next is not None: # this part is really englishic
            slow = slow.next
            fast = fast.next

        if fast is not None:
            slow.next = slow.next.next
            
        else:
            head = head.next
        
        return head