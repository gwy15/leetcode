#
# @lc app=leetcode.cn id=1110 lang=python3
#
# [1110] 删点成林
#
from typing import List
from utils import TreeNode
# @lc code=start


class Solution:
    def delete_nodes(self, root, to_delete) -> (TreeNode, List[TreeNode]):
        if root is None:
            return None, []
        if root.val in to_delete:
            left, left_splitted = self.delete_nodes(root.left, to_delete)
            right, right_splitted = self.delete_nodes(root.right, to_delete)
            root = None
            splitted = left_splitted + right_splitted
            if left:
                splitted.append(left)
            if right:
                splitted.append(right)
        else:
            # split left
            root.left, splitted = self.delete_nodes(root.left, to_delete)
            root.right, _right_splitted = self.delete_nodes(
                root.right, to_delete)
            splitted += _right_splitted

        return root, splitted

    def delNodes(self, root: TreeNode, to_delete: List[int]) -> List[TreeNode]:
        to_delete = set(to_delete)
        root, splitted = self.delete_nodes(root, to_delete)
        if root:
            splitted.append(root)
        return splitted


# @lc code=end
if __name__ == "__main__":
    from utils import Codec
    c = Codec()
    s = Solution()
    ans = s.delNodes(c.deserialize('[1,2,3,4,5,6,7]'), [3, 5])
    print([c.serialize(t) for t in ans])
