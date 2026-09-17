# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def reverse(self, head):
        curr = head
        prev = None

        while curr:
            nxt = curr.next
            curr.next = prev
            prev = curr
            curr = nxt

        return prev

    def reverseKGroup(self, head, k):
        dummy = ListNode(0)
        tail = dummy
        group_head = head

        while group_head:
            kth = group_head

            for _ in range(k - 1):
                if kth is None:
                    break
                kth = kth.next

            if kth is None:
                tail.next = group_head
                break

            group_next = kth.next
            kth.next = None

            new_head = self.reverse(group_head)
            tail.next = new_head

            tail = group_head
            group_head = group_next

        return dummy.next