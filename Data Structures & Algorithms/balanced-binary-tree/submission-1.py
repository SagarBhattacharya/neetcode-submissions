# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    balanced = True

    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        self.height(root)
        return self.balanced
        
    def height(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0
        
        left = self.height(root.left)
        right = self.height(root.right)
        self.balanced = self.balanced and abs(left-right) <= 1

        return 1 + max(left, right)