# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        if not root:
            return []

        stack = [root]
        traversal = []
        while stack:
            curr = stack[-1]
            while curr.left and curr.left.val not in traversal:
                stack.append(curr.left)
                curr = curr.left
            
            curr = stack.pop()
            traversal.append(curr.val)

            if curr.right:
                stack.append(curr.right)
        return traversal