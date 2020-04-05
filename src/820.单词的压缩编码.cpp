/*
 * @lc app=leetcode.cn id=820 lang=cpp
 *
 * [820] 单词的压缩编码
 */

#include <iostream>
#include <string>
#include <vector>

using std::string;
using std::vector;

// @lc code=start

struct Node {
    Node* children[26];

    Node() {
        for (int i = 0; i < 26; i++) children[i] = nullptr;
    }
};

class Solution {
    int sumTree(Node* tree, int height) {
        if (tree == nullptr) {
            return 0;
        }
        int sum = 0;
        for (int i = 0; i < 26; i++) {
            sum += sumTree(tree->children[i], height + 1);
        }
        if (sum == 0) {
            return height + 1;
        } else {
            return sum;
        }
    }

   public:
    int minimumLengthEncoding(vector<string>& words) {
        Node root;
        for (auto& word : words) {
            Node* node = &root;
            for (int i = 0; i < word.length(); i++) {
                int ch = word[word.length() - 1 - i] - 'a';
                // insert into node
                if (node->children[ch] == nullptr) {
                    // std::clog << "new node " << char('a' + ch)
                    //           << " from string " << word << std::endl;
                    node = node->children[ch] = new Node();
                } else {
                    node = node->children[ch];
                }
            }
        }
        return sumTree(&root, 0);
    }
};
// @lc code=end

int main() {
    vector<string> v = {"time", "me", "bell"};
    auto c = Solution().minimumLengthEncoding(v);
    std::cout << c << std::endl;
}
