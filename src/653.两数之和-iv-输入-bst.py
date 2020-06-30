#
# @lc app=leetcode.cn id=653 lang=python3
#
# [653] 两数之和 IV - 输入 BST
#
from utils import TreeNode
# @lc code=start


class Solution:
    def findTarget(self, root: TreeNode, k: int) -> bool:
        def pre(n: TreeNode):
            if n is None:
                return
            for v in pre(n.left):
                yield v
            yield n.val
            for v in pre(n.right):
                yield v

        def rev(n: TreeNode):
            if n is None:
                return
            for v in rev(n.right):
                yield v
            yield n.val
            for v in rev(n.left):
                yield v

        pre_trans = pre(root)
        rev_trans = rev(root)
        left, right = next(pre_trans), next(rev_trans)
        while left < right:
            sum = left + right
            if sum == k:
                return True
            elif sum < k:
                left = next(pre_trans)
            else:
                right = next(rev_trans)
        return False


# @lc code=end
if __name__ == "__main__":
    from utils import Codec
    f = Solution().findTarget
    p = Codec().deserialize
    assert f(p('[5,3,6,2,4,7]'), 9)
    assert not f(p('[5,3,6,2,4,7]'), 28)
