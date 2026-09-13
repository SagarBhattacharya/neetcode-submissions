# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        count, ptr = 0, head
        while ptr:
            count += 1
            ptr = ptr.next
        
        index = count - n
        if index == 0:
            head = head.next
        else:
            ptr = head
            while index > 1:
                ptr = ptr.next
                index -= 1
            ptr.next = ptr.next.next
        return head