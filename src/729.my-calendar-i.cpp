/*
 * @lc app=leetcode id=729 lang=cpp
 *
 * [729] My Calendar I
 */

#include <map>
class MyCalendar {
	std::map<int, int> start2End;

public:
	MyCalendar() : start2End() {}

	bool book(int start, int end) {
		// check the part where start <= part.start
		auto iter = start2End.lower_bound(start);
		if (iter != start2End.end()) {  // next found
			if (iter->first < end) {    // overlaps
				return false;
			}
		}
		if (iter != start2End.cbegin()) {
			iter = std::prev(iter);  // check the part before
			if (iter != start2End.end() && start < iter->second) {
				return false;
			}
		}
		// insert now
		start2End.insert(iter, std::pair<int, int>(start, end));
		return true;
	}
};


/**
 * Your MyCalendar object will be instantiated and called as such:
 * MyCalendar* obj = new MyCalendar();
 * bool param_1 = obj->book(start,end);
 */

