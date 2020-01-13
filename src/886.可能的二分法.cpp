/*
 * @lc app=leetcode.cn id=886 lang=cpp
 *
 * [886] 可能的二分法
 */
#include <iostream>
#include <vector>
#include <queue>
using std::vector;
using std::queue;

// @lc code=start
class Solution {
public:
    bool possibleBipartition(int N, vector<vector<int>>& dislikes) {
        auto edges = vector<vector<int>>(N);
        for (const auto& edge: dislikes) {
            edges[edge[0] - 1].push_back(edge[1] - 1);
            edges[edge[1] - 1].push_back(edge[0] - 1);
        }
        // render color, init as 0 (not colored)
        auto colors = vector<int>(N, 0);
        for (int node=0; node<N; node++) {
            // travel through all nodes
            if (colors[node] == 0) { // not rendered, start a new tree
                if (!tryRenderTree(node, edges, colors)) {
                    return false;
                };
            }
        }
        return true; // success
    }

    // BFS
    bool tryRenderTree(
      int node,
      const vector<vector<int>>& edges,
      vector<int>& colors) {
        
        colors[node] = 1; // init color
        std::queue<int> q; // all nodes are rendered when retrived
        q.push(node);

        while (!q.empty()) {
            // current node
            int node = q.front(); q.pop();
            int color = colors[node];
            // map
            for (auto next: edges[node]) {
                if (colors[next] == color) { // conflict
                    return false;
                } else if (colors[next] == 0) { // not rendered
                    colors[next] = (3 - color); // 1 - 2 swap
                    q.push(next); // check conflict
                } else { // check ok
                    continue;
                }
            }
        }
        return true; // success
    }
};
// @lc code=end

#include <string>
#include <iostream>

int main() {
    const auto N = 50;
    vector<vector<int>> dislikes;
    
    int a, b;
    for (int i=0; i<N; i++) {
        std::cin >> a >> b;
        dislikes.push_back({
            a, b
        });
    }

    Solution().possibleBipartition(N, dislikes);
}

