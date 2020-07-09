import unittest
from typing import List

# begin
from typing import Generator


class TrieNode:
    def __init__(self):
        self.is_node = False
        self.children: List[TrieNode] = [None] * 26

    def __repr__(self):
        return '<{} [{}]>'.format(
            self.is_node,
            ",".join(
                f"{chr(ord('a') + idx)}={child}"
                for idx, child in enumerate(self.children)
                if child is not None
            )
        )


class TrieTree:
    def __init__(self, dictionary: List[str]):
        root = TrieNode()
        for word in dictionary:
            n = root
            for ch in word:
                ch = ord(ch) - ord('a')
                if n.children[ch] is None:
                    n.children[ch] = TrieNode()
                n = n.children[ch]
            n.is_node = True
        self.root = root

    def get_words(self, sentence: str, from_index: int) -> Generator[int, None, None]:
        n = len(sentence)
        i = from_index
        node = self.root
        while i < n:
            ch = sentence[i]
            node = node.children[ord(ch) - ord('a')]
            if node is None:
                return
            if node.is_node:
                yield i - from_index + 1
            i += 1

    def __repr__(self):
        return str(self.root)


class Solution:
    def respace(self, dictionary: List[str], sentence: str) -> int:
        tree = TrieTree(dictionary)
        n = len(sentence)
        f = [0] * (n + 1)
        f[n] = 0
        i = n - 1
        while i >= 0:
            f[i] = 1 + f[i + 1]
            #
            for length in tree.get_words(sentence, i):
                # print(f'found word {sentence[i:i+length]}')
                f[i] = min(f[i], f[i + length])
            i -= 1

        return f[0]
# end


class Test(unittest.TestCase):
    def test(self):
        def f(d, s, ans):
            self.assertEqual(
                Solution().respace(d, s),
                ans
            )
        f(["looked", "just", "like", "her", "brother"],
          "jesslookedjustliketimherbrother", 7)


if __name__ == "__main__":
    unittest.main()
    # def f(d, s, ans):
    #     assert Solution().respace(d, s) == ans
    # f(["looked", "just", "like", "her", "brother"],
    #   "jesslookedjustliketimherbrother", 7)
