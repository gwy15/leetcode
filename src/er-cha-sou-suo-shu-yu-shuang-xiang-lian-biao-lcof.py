
from prelude import *
from prelude import TreeNode as Node


class Solution:
    def treeToDoublyList(self, root: 'Node') -> 'Node':
        if root is None:
            return None
        head = None
        prev = None

        def visit(node: Node):
            nonlocal head, prev
            """"""
            assert node is not None
            # 已经遍历了 node 的左节点了，其直接前驱保存在 prev
            if prev is None:
                head = node
            else:
                node.left = prev
                prev.right = node
            prev = node

        def dfs(node: Node):
            nonlocal head, prev
            if node is None:
                return
            # 中序遍历
            dfs(node.left)
            visit(node)
            dfs(node.right)
            return

        dfs(root)
        # prev 是最后一个节点
        head.left, prev.right = prev, head
        return head


if __name__ == '__main__':
    input = Codec().deserialize('[4,2,5,1,3]')
    calc = Solution().treeToDoublyList(input)
    assert calc.val == 1
    assert calc.right.val == 2
    assert calc.right.right.val == 3
    assert calc.right.right.right.val == 4
    assert calc.right.right.right.right.val == 5
    assert calc.right.right.right.right.right.val == 1
