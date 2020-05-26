/*
 * @lc app=leetcode.cn id=1261 lang=cpp
 *
 * [1261] 在受污染的二叉树中查找元素
 */

// Definition for a binary tree node.
struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

// @lc code=start
class FindElements {
    TreeNode* _root;

    static void recover_tree(TreeNode* root) {
        if (root->left != nullptr) {
            root->left->val = 2 * root->val + 1;
            recover_tree(root->left);
        }
        if (root->right != nullptr) {
            root->right->val = 2 * root->val + 2;
            recover_tree(root->right);
        }
    }

   public:
    FindElements(TreeNode* root) : _root(root) {
        root->val = 0;
        recover_tree(root);
    }

    bool find(int target) {
        // find bit length of target, s.t. target >> length == 1
        target++;
        int length = 0;
        while (target >> length) {
            length++;
        }
        length--;
        // find node
        length--;
        auto ptr = _root;
        while (length >= 0) {
            int bit = (target >> length) & 1;
            if (bit == 1) {
                ptr = ptr->right;
            } else {
                ptr = ptr->left;
            }
            if (ptr == nullptr) {
                return false;
            }

            length--;
        }
        return ptr != nullptr;
    }
};

/**
 * Your FindElements object will be instantiated and called as such:
 * FindElements* obj = new FindElements(root);
 * bool param_1 = obj->find(target);
 */
// @lc code=end
