#
# @lc app=leetcode.cn id=133 lang=python3
#
# [133] 克隆图
#

# Definition for a Node.
class Node:
    def __init__(self, val=0, neighbors=[]):
        self.val = val
        self.neighbors = neighbors
# @lc code=start


class Solution:
    def cloneGraph(self, node: 'Node') -> 'Node':
        if node is None:
            return None
        nodes = {}
        root_val = node.val

        def dfs(node):
            val = node.val
            # new_node is not created
            new_node = Node(val)
            # mark as created
            nodes[val] = new_node
            for next_node in node.neighbors:
                next_val = next_node.val
                if next_val not in nodes:
                    dfs(next_node)
                new_node.neighbors.append(nodes[next_val])

        dfs(node)
        return nodes[root_val]


# @lc code=end
