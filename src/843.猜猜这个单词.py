#
# @lc app=leetcode.cn id=843 lang=python3
#
# [843] 猜猜这个单词
#

# """
# This is Master's API interface.
# You should not implement it, or speculate about its implementation
# """

import unittest
import random
import string
from typing import List

# @lc code=start
LENGTH = 6


def _word_distance(word1, word2) -> int:
    return sum(
        word1[i] == word2[i]
        for i in range(LENGTH))


class Solution:
    def init_dis(self, wordlist: List[str]) -> None:
        self._dis = []
        for i, wi in enumerate(wordlist):
            dis = [
                _word_distance(wi, wordlist[j])
                for j in range(i)
            ]
            self._dis.append(dis)

    def dis(self, i: int, j: int) -> int:
        if i == j:
            return LENGTH
        if i > j:
            return self._dis[i][j]
        return self._dis[j][i]

    def findSecretWord(self, wordlist: List[str], master: 'Master') -> None:
        # init
        n = len(wordlist)
        valid = [True] * n
        self.init_dis(wordlist)
        # find the index i with minimal (maximum count)
        while True:
            minmax_counter = n
            choice_i = 0
            for i in range(n):
                if not valid[i]:
                    continue
                counter = [0] * (LENGTH + 1)
                for j in range(n):
                    if valid[j]:
                        counter[self.dis(i, j)] += 1
                if max(counter) < minmax_counter:
                    minmax_counter = max(counter)
                    choice_i = i
            # now trying this guess
            dis = master.guess(wordlist[choice_i])
            if dis == LENGTH:
                return
            # disabling words
            for j in range(n):
                if valid[j] and self.dis(choice_i, j) != dis:
                    valid[j] = False


# @lc code=end


class Master:
    def __init__(self, options=None, choice=None, max_times=10):
        if options is None:
            options = [
                ''.join(random.choices(string.ascii_lowercase, k=LENGTH))
                for _ in range(100)
            ]
        self.options = options
        if choice is None:
            choice = random.choice(self.options)
        self.choice = choice
        self.max_times = max_times
        self.try_times = 0

    def guess(self, word: str) -> int:
        print('guessing ', word)
        if self.try_times >= self.max_times:
            raise RuntimeError("Max times of guess reached")
        self.try_times += 1
        dis = _word_distance(self.choice, word)
        if dis == LENGTH:
            print('guess correct!')
        return dis


class Test(unittest.TestCase):
    def test_random(self):
        master = Master()
        s = Solution()
        s.findSecretWord(master.options, master)

    def test_1(self):
        master = Master(
            ["gaxckt", "trlccr", "jxwhkz", "ycbfps", "peayuf", "yiejjw", "ldzccp", "nqsjoa", "qrjasy", "pcldos", "acrtag", "buyeia", "ubmtpj", "drtclz", "zqderp", "snywek", "caoztp", "ibpghw", "evtkhl", "bhpfla", "ymqhxk", "qkvipb", "tvmued", "rvbass", "axeasm", "qolsjg", "roswcb", "vdjgxx", "bugbyv", "zipjpc", "tamszl", "osdifo", "dvxlxm", "iwmyfb", "wmnwhe", "hslnop", "nkrfwn", "puvgve", "rqsqpq", "jwoswl", "tittgf", "evqsqe", "aishiv", "pmwovj", "sorbte", "hbaczn", "coifed", "hrctvp", "vkytbw", "dizcxz",
                "arabol", "uywurk", "ppywdo", "resfls", "tmoliy", "etriev", "oanvlx", "wcsnzy", "loufkw", "onnwcy", "novblw", "mtxgwe", "rgrdbt", "ckolob", "kxnflb", "phonmg", "egcdab", "cykndr", "lkzobv", "ifwmwp", "jqmbib", "mypnvf", "lnrgnj", "clijwa", "kiioqr", "syzebr", "rqsmhg", "sczjmz", "hsdjfp", "mjcgvm", "ajotcx", "olgnfv", "mjyjxj", "wzgbmg", "lpcnbj", "yjjlwn", "blrogv", "bdplzs", "oxblph", "twejel", "rupapy", "euwrrz", "apiqzu", "ydcroj", "ldvzgq", "zailgu", "xgqpsr", "wxdyho", "alrplq", "brklfk"],
            "hbaczn",
            10
        )
        s = Solution()
        s.findSecretWord(master.options, master)


unittest.main()
