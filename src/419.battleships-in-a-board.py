#
# @lc app=leetcode id=419 lang=python3
#
# [419] Battleships in a Board
#

# @lc code=start
class Solution:
    def countBattleships(self, board: List[List[str]]) -> int:
        def isHead(i, j):
            if i != 0 and board[i-1][j] == 'X':
                return False
            if j != 0 and board[i][j-1] == 'X':
                return False
            return True
        # scan for head only
        count = 0
        for i, row in enumerate(board):
            for j, pos in enumerate(row):
                if pos == 'X' and isHead(i, j):
                    count += 1
        return count
                      
# @lc code=end

