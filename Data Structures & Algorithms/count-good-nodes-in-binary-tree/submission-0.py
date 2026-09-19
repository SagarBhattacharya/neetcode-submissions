# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    good_nodes = 0

    def goodNodes(self, root: TreeNode) -> int:
        if root is None:
            return 0
        self.dfs(root, root.val)
        return self.good_nodes
    
    def dfs(self, root: TreeNode, maximum):
        if root is None:
            return
        
        if maximum <= root.val:
            self.good_nodes += 1
            maximum = root.val
        
        self.dfs(root.left, maximum)
        self.dfs(root.right, maximum)