# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
from collections import deque

class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if root is None:
            return []

        result = []
        ready_queue = deque([root])
        next_queue = deque()

        while True:
            curr = []
            while ready_queue:
                node = ready_queue.popleft()
                curr.append(node.val)
                if node.left:
                    next_queue.append(node.left)
                if node.right:
                    next_queue.append(node.right)
            
            if curr:
                result.append(curr)

            if not next_queue:
                break
            else:
                ready_queue.extend(next_queue)
                next_queue.clear()

        return result