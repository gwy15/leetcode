#
# @lc app=leetcode.cn id=99 lang=python3
#
# [99] 恢复二叉搜索树
#
from utils import TreeNode
# @lc code=start


class Solution:
    def transversal(self, root: TreeNode):
        if root is None:
            return

        for node in self.transversal(root.left):
            yield node
        yield root
        for node in self.transversal(root.right):
            yield node

    def recoverTree(self, root: TreeNode) -> None:
        """
        Do not return anything, modify root in-place instead.
        """
        i, i_1 = None, None
        j, j_1 = None, None
        last = None
        for node in self.transversal(root):
            if last is None:
                last = node
                continue
            # 找到一个逆序对
            if last.val > node.val:
                # 第一个逆序对
                if i is None:
                    i, i_1 = last, node
                # 第二个
                else:
                    j, j_1 = last, node
                    break

            last = node

        if j is None:
            # 只有一对逆序对，直接交换 j, j_1
            i.val, i_1.val = i_1.val, i.val
        else:
            # 交换 i, j+1
            i.val, j_1.val = j_1.val, i.val


# @lc code=end
if __name__ == "__main__":
    from utils import Codec
    de = Codec().deserialize

    def f(a, b):
        tree = de(a)
        Solution().recoverTree(tree)
        assert tree == de(b)

    f('[1,3,null,null,2]', '[3,1,null,null,2]')
    f('[3,1,4,null,null,2]', '[2,1,4,null,null,3]')
    f('[1,2]', '[2,1]')
