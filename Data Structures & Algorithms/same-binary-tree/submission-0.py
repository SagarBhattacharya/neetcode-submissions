# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        if p is None and q is None:
            return True

        if p and q:
            values_eq = p.val == q.val
            left_result = self.isSameTree(p.left, q.left)
            right_result = self.isSameTree(p.right, q.right)
            return values_eq and left_result and right_result
        else:
            return False