# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


from typing import Tuple

class Solution:
    def increasingBST(self, root: TreeNode) -> TreeNode:
        if root is None:
            return None
        return self.helper(root)[0]
    
    # 返回 (root, rightmost)
    def helper(self, root: TreeNode) -> Tuple[TreeNode, TreeNode]:
        if root is None:
            return None, None
        
        res_rightmost = root
        if root.right:
            root.right, res_rightmost = self.helper(root.right)
        
        if root.left:
            left, left_right = self.helper(root.left)
            root.left = None
            left_right.right = root
            return left, res_rightmost
        else:
            return root, res_rightmost

        


