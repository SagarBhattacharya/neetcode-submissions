# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def lowestCommonAncestor(self, root: TreeNode, p: TreeNode, q: TreeNode) -> TreeNode:
        if self.isDecendent(root, p) and self.isDecendent(root, q):
            if root and self.isDecendent(root.left, p) and self.isDecendent(root.left, q):
                return self.lowestCommonAncestor(root.left, p, q)
            elif root and self.isDecendent(root.right, p) and self.isDecendent(root.right, q):
                return self.lowestCommonAncestor(root.right, p, q)
            else:
                return root
        else:
            return None


    def isDecendent(self, root: TreeNode, n: TreeNode) -> bool:
        if n is None:
            return True
        if root is None:
            return False

        if root.val == n.val:
            return True
        else:
            l = self.isDecendent(root.left, n)
            r = self.isDecendent(root.right, n)
            return l or r