# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def reverse(self, head: Optional[ListNode]):
        curr = head
        prev = None
        while curr:
            next = curr.next
            curr.next = prev
            prev = curr
            curr = next
        return prev

    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        curr = head
        count = k
        dummy = ListNode(0)
        tail = dummy
        group = curr

        while curr:
            if count == 1:
                next = curr.next
                curr.next = None
                rev = self.reverse(group)

                tail.next = rev
                for _ in range(k):
                    tail = tail.next
                group = next

                curr = next
                count = k
            else:
                curr = curr.next
                count -= 1

        tail.next = group
        
        return dummy.next