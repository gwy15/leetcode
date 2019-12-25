/*
 * @lc app=leetcode id=951 lang=kotlin
 *
 * [951] Flip Equivalent Binary Trees
 */

// @lc code=start
/**
 * Example:
 * var ti = TreeNode(5)
 * var v = ti.`val`
 * Definition for a binary tree node.
 * class TreeNode(var `val`: Int) {
 *     var left: TreeNode? = null
 *     var right: TreeNode? = null
 * }
 */
class Solution {
    fun flipEquiv(root1: TreeNode?, root2: TreeNode?): Boolean {
        return if (root1 == null && root2 == null) {
            true
        } else if (root1?.val != root2?.val) {
            false
        } else {
            flipEquiv(root1?.left, root2?.right) && \
            flipEquiv(root1?.right, root2?.left)
        }
    }
}
// @lc code=end

