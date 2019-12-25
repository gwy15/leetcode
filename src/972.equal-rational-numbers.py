#
# @lc app=leetcode id=972 lang=python3
#
# [972] Equal Rational Numbers
#

# @lc code=start
from fractions import Fraction


class Solution:
    def isRationalEqual(self, S: str, T: str) -> bool:
        return self.str2fraction(S) == self.str2fraction(T)

    def str2fraction(self, s: str) -> Fraction:
        bracketPos = s.find('(')
        if bracketPos == -1:
            return Fraction(s)
        else:
            intPart = Fraction(s[:bracketPos])

            baseLength = bracketPos - s.find('.') - 1  # from . (
            rightBracketPos = len(s) - 1
            cycleLength = rightBracketPos - bracketPos - 1
            numerator = int(s[bracketPos+1: rightBracketPos])

            cyclePart = Fraction(
                numerator,
                (10 ** baseLength) * (10 ** cycleLength - 1)
            )

            return intPart + cyclePart

# @lc code=end


print(Solution().isRationalEqual(
    '0.999(99)',  # '0.(52)',
    '1.0',  # '0.5(25)'
))
