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
        addr_map = collections.defaultdict(lambda: Node(0))
        addr_map[None] = None

        curr = head
        while curr:
            addr_map[curr].val = curr.val
            addr_map[curr].next = addr_map[curr.next]
            addr_map[curr].random = addr_map[curr.random]
            curr = curr.next
        return addr_map[head]