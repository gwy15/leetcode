/*
 * @lc app=leetcode.cn id=199 lang=cpp
 *
 * [199] 二叉树的右视图
 */

#include <queue>
#include <vector>
using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};
// @lc code=start
class Solution {
   public:
    vector<int> rightSideView(TreeNode *root) {
        if (root == nullptr) {
            return std::vector<int>();
        }
        std::vector<int> result;
        std::queue<TreeNode *> q;
        q.push(root);
        while (q.size()) {
            int n = q.size();
            TreeNode *element = q.front();
            result.push_back(element->val);
            for (int i = 0; i < n; i++) {
                auto node = q.front();
                if (node->right) {
                    q.push(node->right);
                }
                if (node->left) {
                    q.push(node->left);
                }
                q.pop();
            }
        }
        return result;
    }
};
// @lc code=end
#include <iostream>
int main() {
    TreeNode *root = new TreeNode(1);
    root->left = new TreeNode(2);
    root->left->right = new TreeNode(5);
    root->right = new TreeNode(3);
    // root->right->right = new TreeNode(4);
    for (auto i : Solution().rightSideView(root)) std::cout << i << std::endl;
}