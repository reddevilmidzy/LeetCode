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
    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        ListNode res = new ListNode();
        ListNode nxt = res;
        int carry = 0;

        while (l1 != null || l2 != null || carry != 0) {
            final int digit1 = (l1 != null) ? l1.val : 0;
            final int digit2 = (l2 != null) ? l2.val : 0;

            final int sum = digit1 + digit2 + carry;
            carry = sum / 10;

            ListNode newNode = new ListNode(sum % 10);
            nxt.next = newNode;
            nxt = nxt.next;

            l1 = (l1 != null) ? l1.next : null;
            l2 = (l2 != null) ? l2.next : null;
        }

        res = res.next;
        return res;
    }
}
