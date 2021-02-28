#
# @lc app=leetcode.cn id=640 lang=python3
#
# [640] 求解方程
#

# @lc code=start
class Solution:
    def solveEquation(self, equation: str) -> str:
        """
        可能的状态：
        遇到 x => 前面的系数计入 x，记录正号
        遇到 + => 前面的系数出栈，记录正号
        遇到 - => 前面的系数出栈，记录负号
        遇到 = => 前面的系数出栈，记录rhs
        遇到数字=> 数字继续入栈
        结束 => 前面的系数出栈
        """
        on_left = 1
        k_x = 0
        k_const = 0

        cur_k = None
        cur_neg = 1

        def k():
            if cur_k == None:
                ans = on_left * cur_neg
            else:
                ans = on_left * cur_k * cur_neg
            # print('k = ', ans)
            return ans

        for ch in equation:
            if ch == 'x':
                k_x += k()
                cur_k = None
                cur_neg = 1

            elif ch == '+':
                if cur_k is not None:
                    k_const += k()
                cur_k = None
                cur_neg = 1

            elif ch == '-':
                if cur_k is not None:
                    k_const += k()
                cur_k = None
                cur_neg = -1

            elif ch == '=':
                if cur_k is not None:
                    k_const += k()
                cur_k = None
                cur_neg = 1
                on_left = -1

            else:
                if cur_k is None:
                    cur_k = int(ch)
                else:
                    cur_k = cur_k * 10 + int(ch)
        if cur_k is not None:
            k_const += k()

        # print(equation)
        # print(k_x, k_const)
        # print()

        if k_x == 0:
            if k_const == 0:
                return 'Infinite solutions'
            else:
                return 'No solution'
        else:
            return f'x={-k_const // k_x}'


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().solveEquation(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('x+5-3+x=6+x-2', 'x=2')
    test('x=x', 'Infinite solutions')
    test('2x=x', 'x=0')
    test('2x+3x-6x=x+2', 'x=-1')
    test('x=x+2', 'No solution')
    test('1=0', 'No solution')
    test('1=1', 'Infinite solutions')
    test('x+x-x=x', 'Infinite solutions')
