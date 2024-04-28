/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
    public ListNode mergeKLists(final ListNode[] lists) {
        final Map<Integer, Integer> map = new HashMap<>();
        final Queue<Integer> pq = new PriorityQueue<>();

        for (ListNode head : lists) {
            while (head != null) {
                pq.add(head.val);
                map.put(head.val, map.getOrDefault(head.val, 0) + 1);
                head = head.next;
            }
        }

        ListNode tmp = new ListNode(-1);
        ListNode cur = tmp;

        while (!pq.isEmpty()) {
            final int val = pq.poll();
            ListNode newNode = new ListNode(val);

            cur.next = newNode;
            cur = cur.next;

            final Integer cnt = map.get(val);
            if (cnt != null && cnt > 0) {
                map.put(val, cnt - 1);
            } else {
                map.remove(val);
            }
        }
        return tmp.next;
    }
}
