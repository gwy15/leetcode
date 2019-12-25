#include <string>
#include <vector>
using std::string;
using std::vector;

/*
 * @lc app=leetcode id=71 lang=cpp
 *
 * [71] Simplify Path
 */
class Solution {
   public:
    string simplifyPath(string path) {
        // split
        vector<string> stack;
        string curName = "";
        for (std::size_t i = 0; i < path.size(); i++) {
            switch (char ch = path[i]) {
                case '/':  // break
                    if (curName.size()) {
                        stack.push_back(curName);
                        curName = "";
                    }
                    break;

                default:  // normal char
                    curName += ch;
                    break;
            }
        }
        if (curName.size()) {
            stack.push_back(curName);
        }

        // simplify
        vector<string> simplifiedStack;
        for (std::size_t i = 0; i < stack.size(); i++) {
            if (stack[i] == ".") {
                continue;
            } else if (stack[i] == "..") {
                if (simplifiedStack.size()) simplifiedStack.pop_back();
            } else {
                simplifiedStack.push_back(stack[i]);
            }
        }

        // form result
        string res;
        if (simplifiedStack.size() == 0) {
            res = "/";
        } else {
            for (int i = 0; i < simplifiedStack.size(); i++) {
                res += '/';
                res += simplifiedStack[i];
            }
        }
        return res;
    }
};

// #include <iostream>
// int main() {
//     std::cout << Solution().simplifyPath("/a/b/../../c/") << std::endl;
// }
