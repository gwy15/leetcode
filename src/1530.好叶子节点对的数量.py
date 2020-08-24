#
# @lc app=leetcode.cn id=1530 lang=python3
#
# [1530] 好叶子节点对的数量
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def countPairs(self, root: TreeNode, distance: int) -> int:
        cnt = 0

        def dfs(node: TreeNode) -> List[int]:
            # 返回数组，代表与 node 子树中，与 node 距离为 i 的【叶节点】个数
            nonlocal cnt
            if node is None:
                return [0] * (distance + 1)
            # 找到叶节点
            if node.left is None and node.right is None:
                d = [0] * (distance + 1)
                d[0] = 1
                return d

            left = dfs(node.left)
            right = dfs(node.right)
            # left to right
            for left_dist in range(distance-1):
                max_right_dist = distance - 2 - left_dist
                for i in range(max_right_dist + 1):
                    cnt += left[left_dist] * right[i]
            # return
            dist_array = [0] * (distance + 1)
            for sub_dist in range(distance):
                dist_array[1 + sub_dist] = left[sub_dist] + right[sub_dist]
            return dist_array

        dfs(root)

        return cnt


# @lc code=end
if __name__ == '__main__':
    def test(tree, dist, expected):
        calc = Solution().countPairs(Codec().deserialize(tree), dist)
        if calc != expected:
            print(f'case failed: `{tree, dist}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('[1,2,3,null,4]', 3, 1)
    test('[1,2,3,4,5,6,7]', 3, 2)
    test('[7,1,4,6,null,5,3,null,null,null,null,null,2]', 3, 1)
    test('[100]', 1, 0)
    test('[1,1,1]', 2, 1)
