#
# @lc app=leetcode.cn id=785 lang=python3
#
# [785] 判断二分图
#
from typing import List
# @lc code=start
from collections import defaultdict


class Solution:
    def isBipartite(self, graph: List[List[int]]) -> bool:
        n = len(graph)
        group = {}
        for start in range(n):
            stack = []
            # 找到一个未着色起点
            if start in group:
                continue
            # 随便着色
            group[start] = 0
            stack.append(start)
            # 开始 DFS
            while stack:
                node = stack.pop()
                node_group = group[node]
                for next in graph[node]:
                    if next in group:
                        if group[next] == node_group:
                            return False
                    else:
                        # 未着色的点，继续查找
                        group[next] = 1 - node_group
                        stack.append(next)
        return True


# @lc code=end
if __name__ == "__main__":
    f = Solution().isBipartite
    assert f([[1, 3], [0, 2], [1, 3], [0, 2]])
    assert not f([[1, 2, 3], [0, 2], [0, 1, 3], [0, 2]])
    assert f([[1], [0]])
    assert f([[1], [0], []])
    assert f([[1], [0], [1]])
