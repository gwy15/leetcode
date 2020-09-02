from typing import List
from utils import *
#


class Solution:
    def ser(self, t: TreeNode) -> str:
        def trans(t):
            if t.left:
                for n in trans(t.left):
                    yield str(n)
            yield str(t.val)
            if t.right:
                for n in trans(t.right):
                    yield str(n)
        if t is None:
            return ''
        return ','.join(trans(t))

    def checkSubTree(self, t1: TreeNode, t2: TreeNode) -> bool:
        s1 = self.ser(t1)
        s2 = self.ser(t2)
        return s2 in s1


if __name__ == '__main__':
    def test(input, expected):
        t1, t2 = input
        calc = Solution().checkSubTree(Codec().deserialize(t1), Codec().deserialize(t2))
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(('[1, 2, 3]', '[2]'), True)
    test(('[1, null, 2, 4]', '[3, 2]'), False)
