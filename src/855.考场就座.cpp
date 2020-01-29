/*
 * @lc app=leetcode.cn id=855 lang=cpp
 *
 * [855] 考场就座
 */
#include <cmath>
#include <cstdint>
#include <iostream>
#include <map>
#include <unordered_map>

// @lc code=start
using i32 = std::int32_t;
using i64 = std::int64_t;
using u64 = std::uint64_t;

static const i32 RIGHT_BOUND = std::pow(10, 9) + 100;
static const i32 LEFT_BOUND = -1;

// section [left, right] where left and right are taken
struct Section {
    i32 left;
    i32 right;
    Section(int l, int r) : left(l), right(r) {}
    Section() : left(LEFT_BOUND), right(RIGHT_BOUND) {}
};

std::ostream& operator<<(std::ostream& is, const Section s) {
    is << "[" << s.left << ", " << s.right << "]";
    return is;
}

class ExamRoom {
    i32 N;
    std::map<i64, Section> id_to_section;
    std::unordered_map<i32, Section> pos_to_boundary;

    // middle of the section (where to insert)
    i32 section_candidate(const Section& section) const {
        if (section.left == LEFT_BOUND) {  // left is empty, select pos 0
            return 0;
        } else if (section.right == RIGHT_BOUND) {  // right empty
            return N - 1;
        } else {  // normal
            return section.left + (section.right - section.left) / 2;
        }
    }

    // space between candidate and nearest student (with offset)
    i32 section_value(const Section& section) const {
        // offset is used to keep value >= 0
        static const constexpr i32 OFFSET = 2;
        i32 value;
        if (section.left == LEFT_BOUND) {  // left is empty
            // for [-INF, 1], sit at 0, value 0
            value = OFFSET + section.right - 1;
        } else if (section.right == RIGHT_BOUND) {
            // for [N-2, INF], value is 0 (sit at N-1)
            value = OFFSET + N - 2 - (section.left);
        } else {
            i32 length = section.right - section.left - 1;  // space length
            // 2N => N-1,  2N+1 => N
            if (length <= 0)
                value = OFFSET - 1;
            else
                value = OFFSET + (length - 1) / 2;
        }
        return value;
    }

    // id of the section, sorting by id is sorting by value and candidate
    // position
    i64 id_of_section(const Section& section) const {
        u64 value = section_value(section);
        i64 candidate = section_candidate(section);
        // sort by value then candidate (reversed)
        return (value << 32) + (0xffffffff - candidate);
    }

   public:
    ExamRoom(int N) : N(N) {
        // mark as empty (-INF and INF as empty boundary)
        Section empty_section = Section(LEFT_BOUND, RIGHT_BOUND);
        id_to_section[id_of_section(empty_section)] = empty_section;

        // never mind, not going to use.
        pos_to_boundary[LEFT_BOUND] = Section(LEFT_BOUND, RIGHT_BOUND);
        pos_to_boundary[RIGHT_BOUND] = Section(LEFT_BOUND, RIGHT_BOUND);
    }

    int seat() {
        // select longest section
        auto iter = id_to_section.rbegin();    // sorted by id
        const Section section = iter->second;  // retrive section

        // get new pos
        i32 pos = section_candidate(section);

        // make new sections and remove old one
        id_to_section.erase(id_of_section(section));  // erase the section
        Section left = Section(section.left, pos);    // make new
        Section right = Section(pos, section.right);
        id_to_section[id_of_section(left)] = left;  // insert to map (tree)
        id_to_section[id_of_section(right)] = right;

        // mark point to section
        pos_to_boundary[pos] = section;             // mark pos' left and right
        pos_to_boundary[section.left].right = pos;  // left boundary changed
        pos_to_boundary[section.right].left = pos;  // right changed

        return pos;
    }

    void leave(int p) {
        Section boundary = pos_to_boundary[p];
        //
        Section left = Section(boundary.left, p);
        Section right = Section(p, boundary.right);

        // merge left and right sections
        id_to_section.erase(id_of_section(left));
        id_to_section.erase(id_of_section(right));
        id_to_section[id_of_section(boundary)] = boundary;

        // mark point to section
        pos_to_boundary.erase(p);  // remove p
        pos_to_boundary[boundary.left].right = boundary.right;
        pos_to_boundary[boundary.right].left = boundary.left;
    }
};

// @lc code=end

int main() {
    int N = 4;
    auto room = ExamRoom(N);
    for (int i = 0; i < 4; i++) std::cout << room.seat() << std::endl;
    room.leave(1);
    room.leave(3);
    std::cout << room.seat() << std::endl;
    return 0;
}
