/*
 * @lc app=leetcode.cn id=687 lang=cpp
 *
 * [687] 最长同值路径
 */

struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};

// @lc code=start
#include <algorithm>
#include <tuple>

using namespace std;

class Solution {
public:
  int longestUnivaluePath(TreeNode *root) {
    return max(get<1>(helper(root)) - 1, 0);
  }

  // 返回以 (root 为端点的最长路径, root 下任意最长路径)
  tuple<int, int> helper(TreeNode *root) {
    if (root == nullptr) {
      return {0, 0};
    }
    tuple<int, int> left = helper(root->left);
    int left_start = get<0>(left), left_max = get<1>(left);
    tuple<int, int> right = helper(root->right);
    int right_start = get<0>(right), right_max = get<1>(right);

    // 计算以 root 为端点的嘴长距离
    int root_start = 1;
    if (root->left && root->val == root->left->val) {
      root_start = max(root_start, 1 + get<0>(left));
    }
    if (root->right && root->val == root->right->val) {
      root_start = max(root_start, 1 + get<0>(right));
    }

    int tree_max = 1;
    tree_max = max(tree_max, left_max);
    tree_max = max(tree_max, right_max);
    tree_max = max(tree_max, root_start);

    if (root->left && root->val == root->left->val && root->right &&
        root->val == root->right->val) {
      tree_max = max(tree_max, 1 + left_start + right_start);
    }

    return {root_start, tree_max};
  }
};
// @lc code=end

int main() { Solution sol{}; }