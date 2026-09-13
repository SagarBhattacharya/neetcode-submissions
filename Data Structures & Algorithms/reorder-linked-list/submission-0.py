# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        mid, ptr = head, head
        while ptr and ptr.next:
            mid = mid.next
            ptr = ptr.next.next

        ptr = mid.next
        rev = mid.next = None
        while ptr:
            next = ptr.next
            ptr.next = rev
            rev = ptr
            ptr = next

        first, second = head, rev
        while second:
            next1, next2 = first.next, second.next
            first.next = second
            second.next = next1
            first, second = next1, next2
