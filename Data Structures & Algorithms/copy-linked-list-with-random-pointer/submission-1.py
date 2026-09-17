"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""

class Solution:
    def copyRandomList(self, head: Optional[Node]) -> Optional[Node]:
        addr_map = {}
        curr = head
        while curr:
            new_node = Node(curr.val)
            addr_map[curr] = new_node
            curr = curr.next

        curr = head
        while curr:
            addr_map[curr].next = addr_map[curr.next] if curr.next else None
            addr_map[curr].random = addr_map[curr.random] if curr.random else None
            curr = curr.next

        return addr_map[head] if head else None
