class Solution:
    def nodesBetweenCriticalPoints(self, head: Optional[ListNode]) -> List[int]:
        res = [-1, -1]

        min_distance = float("inf")

        pre_node = head
        cur_node = head.next
        cur_idx = 1
        pre_idx = 0
        tmp = 0

        while cur_node.next is not None:
            if (
                cur_node.val < pre_node.val
                and cur_node.val < cur_node.next.val
            ) or (
                cur_node.val > pre_node.val
                and cur_node.val > cur_node.next.val
            ):

                if pre_idx == 0:
                    pre_idx = cur_idx
                    tmp = cur_idx
                else:
                    min_distance = min(
                        min_distance, cur_idx - pre_idx
                    )
                    pre_idx = cur_idx

            cur_idx += 1
            pre_node = cur_node
            cur_node = cur_node.next

        if min_distance != float("inf"):
            max_distance = pre_idx - tmp
            res = [min_distance, max_distance]

        return res
