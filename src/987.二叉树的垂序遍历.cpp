/*
 * @lc app=leetcode.cn id=987 lang=cpp
 *
 * [987] 二叉树的垂序遍历
 */

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

// @lc code=start
#include <algorithm>
#include <tuple>
#include <vector>
using namespace std;

struct Column {
    Column *left, *right;
    vector<tuple<int, int>> nodes;

    Column() : left(nullptr), right(nullptr), nodes() {}
};

class Solution {
    void visit(TreeNode *root, Column *col, int y) {
        col->nodes.push_back(make_tuple(y, root->val));
        if (root->left) {
            if (col->left == nullptr) {
                col->left = new Column();
                col->left->right = col;
            }
            visit(root->left, col->left, y + 1);
        }
        if (root->right) {
            if (col->right == nullptr) {
                col->right = new Column();
                col->right->left = col;
            }
            visit(root->right, col->right, y + 1);
        }
    }

   public:
    vector<vector<int>> verticalTraversal(TreeNode *root) {
        Column *c = new Column();
        visit(root, c, 0);

        while (c->left != nullptr) {
            c = c->left;
        }

        auto ans = vector<vector<int>>();
        //
        while (c != nullptr) {
            //
            vector<int> column = vector<int>();
            auto nodes = std::move(c->nodes);
            std::sort(nodes.begin(), nodes.end());
            for (auto tuple : nodes) {
                column.push_back(std::get<1>(tuple));
            }
            ans.push_back(std::move(column));
            c = c->right;
        }

        return std::move(ans);
    }
};
// @lc code=end
int main() {
    auto root = new TreeNode(3);
    root->left = new TreeNode(9);
    root->right = new TreeNode(20);
    root->right->left = new TreeNode(15);
    root->right->right = new TreeNode(7);

    Solution().verticalTraversal(root);
}