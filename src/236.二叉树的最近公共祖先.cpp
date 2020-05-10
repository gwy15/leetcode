/*
 * @lc app=leetcode.cn id=236 lang=cpp
 *
 * [236] 二叉树的最近公共祖先
 */

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};
// @lc code=start
class Solution {
   public:
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        if (root == nullptr || root == p || root == q) {
            return root;
        }

        TreeNode* left_result = lowestCommonAncestor(root->left, p, q);
        TreeNode* right_result = lowestCommonAncestor(root->right, p, q);
        if (left_result && right_result) {
            return root;
        }
        return (TreeNode*)((size_t)left_result | (size_t)right_result);
    }
};
// @lc code=end
