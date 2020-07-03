#
# @lc app=leetcode.cn id=108 lang=python3
#
# [108] 将有序数组转换为二叉搜索树
#
from utils import TreeNode
from typing import List
# @lc code=start


class Solution:
    def array_to_bst(self, nums: List[int], start: int, end: int) -> TreeNode:
        n = end - start
        if n <= 0:
            return None

        mid = start + (n // 2)
        root_val = nums[mid]
        root = TreeNode(root_val)
        root.left = self.array_to_bst(nums, start, mid)
        root.right = self.array_to_bst(nums, mid+1, end)
        return root

    def sortedArrayToBST(self, nums: List[int]) -> TreeNode:
        return self.array_to_bst(nums, 0, len(nums))


# @lc code=end
if __name__ == "__main__":
    from utils import Codec
    f = Solution().sortedArrayToBST
    p = Codec().deserialize
    assert f([-10, -3, 0, 5, 9]) == p('[0,-3,9,-10,null,5]')
    assert f([]) == None
