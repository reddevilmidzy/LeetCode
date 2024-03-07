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
    public ListNode middleNode(ListNode head) {
        int len = length(head);
        int index = 0;
        while (index != len / 2) {
            head = head.next;
            index++;
        }
        return head;
    }

    private int length(ListNode head) {
        int res = 0;
        while (head != null) {
            head = head.next;
            res++;
        }
        return res;
    }
}