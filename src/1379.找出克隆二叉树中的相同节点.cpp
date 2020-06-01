/*
 * @lc app=leetcode.cn id=1379 lang=cpp
 *
 * [1379] 找出克隆二叉树中的相同节点
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
  TreeNode* getTargetCopy(TreeNode* original, TreeNode* cloned,
                          TreeNode* target) {
    if (original == nullptr) {
      return nullptr;
    }
    if (target == original) {
      return cloned;
    }
    auto ans = getTargetCopy(original->left, cloned->left, target);
    if (ans != nullptr) return ans;
    return getTargetCopy(original->right, cloned->right, target);
  }
};
// @lc code=end
