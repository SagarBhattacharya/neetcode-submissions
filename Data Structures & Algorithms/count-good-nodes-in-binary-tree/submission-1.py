# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:

    def goodNodes(self, root: TreeNode) -> int:
        return self.dfs(root, root.val)
    
    def dfs(self, root: TreeNode, maximum) -> int:
        if root is None:
            return 0
        
        res = 1 if maximum <= root.val else 0
        next_max = max(maximum, root.val)        
        res += self.dfs(root.left, next_max)
        res += self.dfs(root.right, next_max)
        return res