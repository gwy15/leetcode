#
# @lc app=leetcode id=290 lang=python3
#
# [290] Word Pattern
#

# @lc code=start
class Solution:
    def wordPattern(self, pattern: str, str: str) -> bool:
        strings = str.split(' ')
        if len(pattern) != len(strings):
            return False
        pattern_to_str = {}
        str_to_pattern = {}
        for i in range(len(pattern)):
            p = pattern[i] 
            s = strings[i]
            if p in pattern_to_str:
                if pattern_to_str[p] != s:
                    return False
                if str_to_pattern.get(s, None) != p:
                    return False
            else:
                if s in str_to_pattern:
                    return False
                pattern_to_str[p] = s
                str_to_pattern[s] = p
            
        return True
        
# @lc code=end

