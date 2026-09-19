# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
from collections import deque

class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        if root is None: return []
        result = []
        queue = deque([root])

        while queue:
            node_count = len(queue)
            rightmost = None
            for i in range(node_count):
                node = queue.popleft()
                rightmost = node.val
                if node.left:
                    queue.append(node.left)
                if node.right:
                    queue.append(node.right)
            if rightmost:
                result.append(rightmost)
        return result