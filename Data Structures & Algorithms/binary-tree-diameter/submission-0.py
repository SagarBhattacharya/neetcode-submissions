# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    max_height = 0

    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        self.height(root)
        return self.max_height

    def height(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0

        left_h = self.height(root.left)
        right_h = self.height(root.right)
        self.max_height = max(self.max_height, left_h + right_h)

        return max(left_h, right_h) + 1

        