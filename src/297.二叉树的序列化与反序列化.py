#
# @lc app=leetcode.cn id=297 lang=python3
#
# [297] 二叉树的序列化与反序列化
#

# Definition for a binary tree node.


class TreeNode(object):
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None

    def __eq__(self, rhs):
        if rhs is None:
            return False
        if self.val != rhs.val:
            return False
        return self.left == rhs.left and self.right == rhs.right

    def __repr__(self):
        return f'(V={self.val} L={self.left} R={self.right})'
# @lc code=start


class Codec:
    def serialize(self, root):
        """Encodes a tree to a single string.

        :type root: TreeNode
        :rtype: str
        """
        import queue
        q = queue.Queue()
        q.put(root)
        ans = []
        while q.qsize():
            n = q.qsize()
            for _ in range(n):
                node: TreeNode = q.get()
                if node is None:
                    ans.append('null')
                else:
                    ans.append(str(node.val))
                    q.put(node.left)
                    q.put(node.right)
        while len(ans) and ans[-1] == 'null':
            ans.pop()
        return '[' + ','.join(ans) + ']'

    def deserialize(self, s):
        """Decodes your encoded data to tree.

        :type data: str
        :rtype: TreeNode
        """
        import queue
        if s == '[]':
            return None

        data = queue.Queue()
        for item in s[1:-1].split(','):
            if item == 'null':
                data.put(None)
            else:
                data.put(int(item))

        root = TreeNode(data.get())
        q = queue.Queue()
        q.put(root)
        while q.qsize():
            node = q.get()
            # make left tree
            if data.qsize():
                left = data.get()
                if left is not None:
                    node.left = TreeNode(left)
                    q.put(node.left)
            else:
                break
            # make right tree
            if data.qsize():
                right = data.get()
                if right is not None:
                    node.right = TreeNode(right)
                    q.put(node.right)
            else:
                break
        return root

# @lc code=end


class Test:
    def test_ser(self):
        c = Codec()
        # []
        assert c.serialize(None) == '[]'
        assert c.deserialize('[]') == None
        #
        t = TreeNode(1)
        t.left = TreeNode(2)
        t.right = TreeNode(3)
        assert c.serialize(t) == '[1,2,3]'
        assert c.deserialize('[1,2,3]') == t
        #
        t = TreeNode(1)
        t.right = TreeNode(2)
        t.right.left = TreeNode(3)
        assert c.serialize(t) == '[1,null,2,3]'
        assert c.deserialize('[1,null,2,3]') == t
        #
        t = TreeNode(5)
        t.left = TreeNode(4)
        t.left.left = TreeNode(3)
        t.left.left.left = TreeNode(-1)
        t.right = TreeNode(7)
        t.right.left = TreeNode(2)
        t.right.left.left = TreeNode(9)
        assert c.serialize(t) == '[5,4,7,3,null,2,null,-1,null,9]'
        assert c.deserialize('[5,4,7,3,null,2,null,-1,null,9]') == t
