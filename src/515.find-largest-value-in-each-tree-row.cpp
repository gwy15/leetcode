/*
 * @lc app=leetcode id=515 lang=cpp
 *
 * [515] Find Largest Value in Each Tree Row
 */

#include <vector>
#include <algorithm>
#include <queue>
using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

// @lc code=start

class Solution {
   public:
    vector<int> largestValues(TreeNode *root) {
        std::vector<int> result;
        if (root == nullptr) {
            return result;
        }
        std::queue<TreeNode*> q;
        q.push(root);
        while (q.size()) {
            auto qsize = q.size();
            auto line_max = q.front()->val;
            for (int i=0; i<qsize; i++) {
                auto node = q.front();
                q.pop();
                line_max = max(line_max, node->val);
                if (node->left)
                    q.push(node->left);
                if (node->right)
                    q.push(node->right);
            }
            result.push_back(line_max);
        }

        return result;
    }
};
// @lc code=end
