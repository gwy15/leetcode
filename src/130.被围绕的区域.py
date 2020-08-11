#
# @lc app=leetcode.cn id=130 lang=python3
#
# [130] 被围绕的区域
#
from typing import List
# @lc code=start


class Solution:
    def solve(self, board: List[List[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        if len(board) == 0:
            return
        DELTA = ((-1, 0), (0, 1), (1, 0), (0, -1))
        m, n = len(board), len(board[0])
        # border O => B

        def dfs(i, j):
            board[i][j] = 'B'
            for di, dj in DELTA:
                ii, jj = i + di, j + dj
                if 0 <= ii < m and 0 <= jj < n and board[ii][jj] == 'O':
                    dfs(ii, jj)

        for i in range(m):
            for j in (0, n-1):
                if board[i][j] == 'O':
                    dfs(i, j)
        for j in range(1, n-1):
            for i in (0, m-1):
                if board[i][j] == 'O':
                    dfs(i, j)

        # O => X
        for i in range(m):
            for j in range(n):
                if board[i][j] == 'O':
                    board[i][j] = 'X'
        # B => O
        for i in range(m):
            for j in range(n):
                if board[i][j] == 'B':
                    board[i][j] = 'O'


# @lc code=end
if __name__ == "__main__":
    def test(mat, ans):
        mat = [list(s) for s in mat]
        Solution().solve(mat)
        mat = [''.join(ss) for ss in mat]
        if mat != ans:
            print('mat:    \tans:')
            for s_mat, s_ans in zip(mat, ans):
                if s_mat != s_ans:
                    print(f'> {s_mat}\t  {s_ans}')
                else:
                    print(f'  {s_mat}\t  {s_ans}')
        assert mat == ans

    test([
        'XXXX',
        'XOOX',
        'XXOX',
        'XOXX'
    ], [
        'XXXX', 'XXXX', 'XXXX', 'XOXX'
    ]
    )

    test([
        'XOOXXXOXOO',
        'XOXXXXXXXX',
        'XXXXOXXXXX',
        'XOXXXOXXXO',
        'OXXXOXOXOX',
        'XXOXXOOXXX',
        'OXXOOXOXXO',
        'OXXXXXOXXX',
        'XOOXXOXXOO',
        'XXXOOXOXXO'
    ], [
        'XOOXXXOXOO',
        'XOXXXXXXXX',
        'XXXXXXXXXX',
        'XXXXXXXXXO',
        'OXXXXXXXXX',
        'XXXXXXXXXX',
        'OXXXXXXXXO',
        'OXXXXXXXXX',
        'XXXXXXXXOO',
        'XXXOOXOXXO'])
