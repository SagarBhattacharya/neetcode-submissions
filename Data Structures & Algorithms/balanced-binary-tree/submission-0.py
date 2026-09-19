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

        lh = self.height(root.left)
        rh = self.height(root.right)

        if self.balanced and abs(lh - rh) > 1:
            self.balanced = False
        
        return max(lh, rh) + 1
