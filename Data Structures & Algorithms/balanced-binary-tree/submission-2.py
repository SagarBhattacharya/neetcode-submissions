# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        if root is None:
            return True

        stack = [(root, False)]
        heights = {}

        while stack:
            node, visited = stack.pop()

            if not visited:
                stack.append((node, True))

                if node.right:
                    stack.append((node.right, False))

                if node.left:
                    stack.append((node.left, False))

            else:
                left = heights.get(node.left, 0)
                right = heights.get(node.right, 0)

                if abs(left - right) > 1:
                    return False

                heights[node] = 1 + max(left, right)

        return True