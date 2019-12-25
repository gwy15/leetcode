/*
 * @lc app=leetcode id=871 lang=cpp
 *
 * [871] Minimum Number of Refueling Stops
 */

#include <algorithm>
#include <vector>
using std::vector;

// @lc code=start
class Solution {
   public:
    int minRefuelStops(int target, int startFuel,
                       vector<vector<int>>& stations) {
        int fuel = startFuel;
        int pos = 0;
        int refuelTimes = 0;
        stations.push_back({target, 0});
        std::vector<int> backupFuel;
        for (int i = 0; i < stations.size(); i++) {
            int stationPos = stations[i][0];
            int stationFuel = stations[i][1];
            // go!
            while (stationPos - pos > fuel) {  // too far
                if (backupFuel.size()) {
                    // get some refuel!
                    std::pop_heap(backupFuel.begin(), backupFuel.end());
                    fuel += backupFuel.back();
                    backupFuel.pop_back();
                    refuelTimes += 1;
                } else { // all backup fuels are used!
                    return -1;
                }
            }
            fuel -= stationPos - pos;
            pos = stationPos;

            // reach fuel station, put to backup
            backupFuel.push_back(stationFuel);
            std::push_heap(backupFuel.begin(), backupFuel.end());
        }
        return refuelTimes;
    }
};
// @lc code=end
