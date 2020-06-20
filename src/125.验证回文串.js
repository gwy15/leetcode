/*
 * @lc app=leetcode.cn id=125 lang=javascript
 *
 * [125] 验证回文串
 */

// @lc code=start

function is_valid_char(char) {
    let c = char.toLowerCase();
    return (
        "a" <= c && c <= "z"
    ) || (
        "0" <= c && c <= "9"
    );
}

/**
 * @param {string} s
 * @return {boolean}
 */
var isPalindrome = function (s) {
    let n = s.length;
    let left = 0, right = n - 1;
    while (left < right) {
        while (left < right && !is_valid_char(s[left])) {
            left++;
        }
        while (left < right && !is_valid_char(s[right])) {
            right--;
        }
        if (s[left].toLowerCase() != s[right].toLowerCase()) {
            return false;
        }
        left++;
        right--;
    }
    return true;
};
// @lc code=end

