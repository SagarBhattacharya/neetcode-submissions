# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
from collections import deque

class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        result = []
        queue = deque([root])

        while queue: 
            node_count = len(queue)
            level = []
            for i in range(node_count):
                node = queue.popleft()
                if node: 
                    level.append(node.val)
                    queue.append(node.left)
                    queue.append(node.right)
            if level:
                result.append(level)
        return result
