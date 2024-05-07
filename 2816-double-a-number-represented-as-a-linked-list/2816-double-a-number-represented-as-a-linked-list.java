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
    public ListNode doubleIt(ListNode head) {
        Stack<Integer> stk = new Stack<>();
        int val = 0;

        while (head != null) {
            stk.push(head.val);
            head = head.next;
        }

        ListNode newTail = null;

        while (!stk.isEmpty() || val != 0) {
            newTail = new ListNode(0, newTail);

            if (!stk.isEmpty()) {
                val += stk.pop() * 2;
            }
            newTail.val = val % 10;
            val /= 10;
        }

        return newTail;
    }
}
